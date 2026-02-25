use std::io::{self, Read, Write};
use std::time::Duration;

use kplatform_core::{Event, Platform, PlatformError, WindowConfig};

/// Guard to enter alternate screen and hide cursor; restores on drop.
#[derive(Debug)]
struct TermGuard {
    // Optional saved terminal state (if stdin is a TTY and termios available)
    saved: Option<SavedTerm>,
}

#[derive(Copy, Clone, Debug)]
struct SavedTerm {
    term: Termios,
    flags: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

#[allow(non_camel_case_types)]
type c_int = i32;

unsafe extern "C" {
    fn tcgetattr(fd: c_int, termios_p: *mut Termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const Termios) -> c_int;
    fn cfmakeraw(termios_p: *mut Termios);
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
}

const TCSANOW: c_int = 0;
const F_GETFL: c_int = 3;
const F_SETFL: c_int = 4;
const O_NONBLOCK: c_int = 0x800; // Linux

impl TermGuard {
    fn emit_enter(mut w: impl Write) -> io::Result<()> {
        // Enter alternate screen, hide cursor, clear screen, move home
        // ESC[?1049h = alt screen, ESC[?25l = hide cursor, ESC[2J = clear, ESC[H = home
        w.write_all(b"\x1b[?1049h\x1b[?25l\x1b[2J\x1b[H")?;
        Ok(())
    }

    fn emit_leave(mut w: impl Write) -> io::Result<()> {
        // Show cursor, reset attributes, leave alternate screen
        // ESC[?25h = show cursor, ESC[0m = reset, ESC[?1049l = leave alt screen
        w.write_all(b"\x1b[?25h\x1b[0m\x1b[?1049l")?;
        Ok(())
    }

    fn install() -> io::Result<Self> {
        let out = io::stdout();
        let mut lock = out.lock();
        Self::emit_enter(&mut lock)?;
        lock.flush()?;
        // Try to enable raw mode + non-blocking on stdin (fd=0)
        let fd: c_int = 0;
        let mut saved: Option<SavedTerm> = None;
        unsafe {
            // Read current termios
            let mut cur = Termios {
                c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0u8; 32],
                c_ispeed: 0,
                c_ospeed: 0,
            };
            if tcgetattr(fd, &mut cur as *mut Termios) == 0 {
                let orig = cur;
                cfmakeraw(&mut cur as *mut Termios);
                let _ = tcsetattr(fd, TCSANOW, &cur as *const Termios);
                // Set O_NONBLOCK on fd
                let flags = fcntl(fd, F_GETFL);
                if flags >= 0 {
                    let _ = fcntl(fd, F_SETFL, flags | O_NONBLOCK);
                    saved = Some(SavedTerm { term: orig, flags });
                }
            }
        }
        Ok(Self { saved })
    }
}

impl Drop for TermGuard {
    fn drop(&mut self) {
        // Restore termios and flags if we changed them
        if let Some(st) = self.saved {
            unsafe {
                let _ = tcsetattr(0, TCSANOW, &st.term as *const Termios);
                let _ = fcntl(0, F_SETFL, st.flags);
            }
        }
        // Leave alternate screen and show cursor
        let out = io::stdout();
        let mut lock = out.lock();
        let _ = Self::emit_leave(&mut lock);
        let _ = lock.flush();
    }
}

/// Minimal terminal-backed platform. Renders using ANSI truecolor escapes.
#[derive(Debug)]
pub struct TermPlatform {
    size: (u32, u32),
    _guard: TermGuard,
    input_buf: Vec<u8>,
}
impl TermPlatform {
    pub fn new(config: &WindowConfig) -> Result<Self, PlatformError> {
        let size = (config.width, config.height);
        let guard = TermGuard::install().map_err(PlatformError::Io)?;
        Ok(Self {
            size,
            _guard: guard,
            input_buf: Vec::with_capacity(64),
        })
    }

    fn write_row_truecolor(mut w: impl Write, row: &[u32]) -> io::Result<()> {
        if row.is_empty() {
            w.write_all(b"\x1b[0m\n")?;
            return Ok(());
        }
        let mut cur = row[0].to_le_bytes();
        let mut run = 1usize;
        write!(w, "\x1b[48;2;{};{};{}m", cur[0], cur[1], cur[2])?;
        for &px in &row[1..] {
            let rgb = px.to_le_bytes();
            if rgb[..3] == cur[..3] {
                run += 1;
            } else {
                // flush spaces for previous color
                Self::write_spaces(&mut w, run)?;
                // switch color
                write!(w, "\x1b[48;2;{};{};{}m", rgb[0], rgb[1], rgb[2])?;
                cur = rgb;
                run = 1;
            }
        }
        // final run
        Self::write_spaces(&mut w, run)?;
        w.write_all(b"\x1b[0m\n")?;
        Ok(())
    }

    fn write_spaces(mut w: impl Write, mut len: usize) -> io::Result<()> {
        const BUF: [u8; 64] = [b' '; 64];
        while len > 0 {
            let n = len.min(BUF.len());
            w.write_all(&BUF[..n])?;
            len -= n;
        }
        Ok(())
    }

    fn present_to_writer(
        &mut self,
        width: u32,
        height: u32,
        pixels_rgba_le: &[u32],
        mut w: impl Write,
    ) -> io::Result<()> {
        let (w_u, h_u) = (width as usize, height as usize);
        let count = w_u.saturating_mul(h_u);
        if pixels_rgba_le.len() < count {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "pixels buffer smaller than width*height",
            ));
        }

        // Build full frame into a buffer to avoid partial writes,
        // then write with retry handling for WouldBlock.
        let mut out = Vec::with_capacity(w_u.saturating_mul(12).saturating_mul(h_u).max(1024));
        out.extend_from_slice(b"\x1b[H");
        for y in 0..h_u {
            let row = &pixels_rgba_le[y * w_u..y * w_u + w_u];
            Self::write_row_truecolor(&mut out, row)?;
        }
        Self::robust_write_all(&mut w, &out)?;
        Self::robust_flush(&mut w)
    }
}

impl TermPlatform {
    fn robust_write_all(w: &mut impl Write, buf: &[u8]) -> io::Result<()> {
        let mut off = 0;
        while off < buf.len() {
            match w.write(&buf[off..]) {
                Ok(0) => {
                    return Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "failed to write all bytes",
                    ));
                }
                Ok(n) => off += n,
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    std::thread::yield_now();
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
    fn robust_flush(w: &mut impl Write) -> io::Result<()> {
        loop {
            match w.flush() {
                Ok(()) => return Ok(()),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    std::thread::yield_now();
                }
                Err(e) => return Err(e),
            }
        }
    }
    fn parse_event(buf: &[u8]) -> Option<(Event, usize)> {
        if buf.is_empty() {
            return None;
        }
        let b0 = buf[0];
        match b0 {
            0x1b => {
                if buf.len() >= 3 && buf[1] == b'[' {
                    match buf[2] {
                        b'A' => Some((Event::KeyDown(kplatform_core::Key::Up), 3)),
                        b'B' => Some((Event::KeyDown(kplatform_core::Key::Down), 3)),
                        b'C' => Some((Event::KeyDown(kplatform_core::Key::Right), 3)),
                        b'D' => Some((Event::KeyDown(kplatform_core::Key::Left), 3)),
                        _ => None,
                    }
                } else {
                    Some((Event::KeyDown(kplatform_core::Key::Escape), 1))
                }
            }
            b'\r' | b'\n' => Some((Event::KeyDown(kplatform_core::Key::Enter), 1)),
            0x7f | 0x08 => Some((Event::KeyDown(kplatform_core::Key::Backspace), 1)),
            b @ 0x20..=0x7e => Some((Event::KeyDown(kplatform_core::Key::Char(b as char)), 1)),
            _ => None,
        }
    }
}

impl Platform for TermPlatform {
    fn size(&self) -> (u32, u32) {
        self.size
    }

    fn poll_event(&mut self) -> Option<Event> {
        // Read any available bytes from stdin (non-blocking)
        let mut tmp = [0u8; 256];
        let stdin = io::stdin();
        let mut lock = stdin.lock();
        loop {
            match lock.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => self.input_buf.extend_from_slice(&tmp[..n]),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }
        if self.input_buf.is_empty() {
            return None;
        }
        if let Some((ev, used)) = Self::parse_event(&self.input_buf) {
            self.input_buf.drain(0..used);
            return Some(ev);
        }
        None
    }

    fn present_rgba_le(
        &mut self,
        width: u32,
        height: u32,
        pixels_rgba_le: &[u32],
    ) -> Result<(), PlatformError> {
        let mut out = io::stdout();
        self.present_to_writer(width, height, pixels_rgba_le, &mut out)
            .map_err(PlatformError::Io)
    }

    fn sleep(&self, dt: Duration) {
        std::thread::sleep(dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kplatform_core::WindowConfig;

    #[test]
    fn present_one_pixel_generates_escape() {
        let cfg = WindowConfig::new("term", 1, 1);
        let mut term = TermPlatform::new(&cfg).unwrap();
        let px = [u32::from_le_bytes([10, 20, 30, 255])];
        let mut buf = Vec::new();
        term.present_to_writer(1, 1, &px, &mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("\u{1b}[48;2;10;20;30m"));
        assert!(s.contains("\u{1b}[0m"));
    }

    #[test]
    fn write_row_truecolor_rle() {
        let a = u32::from_le_bytes([1, 2, 3, 255]);
        let b = u32::from_le_bytes([9, 8, 7, 255]);
        let c = u32::from_le_bytes([50, 60, 70, 255]);
        let row = [a, a, a, b, b, c, c, c, c];
        let mut v = Vec::new();
        TermPlatform::write_row_truecolor(&mut v, &row).unwrap();
        let s = String::from_utf8(v).unwrap();
        assert_eq!(s.matches("\u{1b}[48;2;").count(), 3);
        assert!(s.ends_with("\u{1b}[0m\n"));
        let spaces = s.chars().filter(|&ch| ch == ' ').count();
        assert_eq!(spaces, row.len());
    }

    #[test]
    fn term_guard_sequences() {
        let mut buf = Vec::new();
        TermGuard::emit_enter(&mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert!(s.contains("\u{1b}[?1049h"));
        assert!(s.contains("\u{1b}[?25l"));
        assert!(s.contains("\u{1b}[2J"));
        assert!(s.contains("\u{1b}[H"));

        let mut buf2 = Vec::new();
        TermGuard::emit_leave(&mut buf2).unwrap();
        let s2 = String::from_utf8(buf2).unwrap();
        assert!(s2.contains("\u{1b}[?25h"));
        assert!(s2.contains("\u{1b}[0m"));
        assert!(s2.contains("\u{1b}[?1049l"));
    }

    #[test]
    fn parse_basic_keys() {
        use kplatform_core::Key;
        let e = TermPlatform::parse_event(b"a").unwrap();
        assert!(matches!(e.0, Event::KeyDown(Key::Char('a'))));
        let e = TermPlatform::parse_event(b"\n").unwrap();
        assert!(matches!(e.0, Event::KeyDown(Key::Enter)));
        let e = TermPlatform::parse_event(b"\x7f").unwrap();
        assert!(matches!(e.0, Event::KeyDown(Key::Backspace)));
    }

    #[test]
    fn parse_arrows_and_escape() {
        use kplatform_core::Key;
        let e = TermPlatform::parse_event(b"\x1b[A").unwrap();
        assert!(matches!(e.0, Event::KeyDown(Key::Up)));
        let e = TermPlatform::parse_event(b"\x1b[B").unwrap();
        assert!(matches!(e.0, Event::KeyDown(Key::Down)));
        let e = TermPlatform::parse_event(b"\x1b[C").unwrap();
        assert!(matches!(e.0, Event::KeyDown(Key::Right)));
        let e = TermPlatform::parse_event(b"\x1b[D").unwrap();
        assert!(matches!(e.0, Event::KeyDown(Key::Left)));
        let e = TermPlatform::parse_event(b"\x1b").unwrap();
        assert!(matches!(e.0, Event::KeyDown(Key::Escape)));
    }
}
