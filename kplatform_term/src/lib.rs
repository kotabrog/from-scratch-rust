use std::io::{self, Write};
use std::time::Duration;

use kplatform_core::{Event, Platform, PlatformError, WindowConfig};

/// Guard to enter alternate screen and hide cursor; restores on drop.
#[derive(Debug)]
struct TermGuard;

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
        Ok(Self)
    }
}

impl Drop for TermGuard {
    fn drop(&mut self) {
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
}
impl TermPlatform {
    pub fn new(config: &WindowConfig) -> Result<Self, PlatformError> {
        let size = (config.width, config.height);
        let guard = TermGuard::install().map_err(PlatformError::Io)?;
        Ok(Self {
            size,
            _guard: guard,
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

        // Move cursor to home before full redraw
        w.write_all(b"\x1b[H")?;

        for y in 0..h_u {
            let row = &pixels_rgba_le[y * w_u..y * w_u + w_u];
            Self::write_row_truecolor(&mut w, row)?;
        }
        w.flush()
    }
}

impl Platform for TermPlatform {
    fn size(&self) -> (u32, u32) {
        self.size
    }

    fn poll_event(&mut self) -> Option<Event> {
        // MVP: no input yet. Future work: raw mode + parser.
        let _ = self.size; // silence potential lint of not using self fields later
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
}
