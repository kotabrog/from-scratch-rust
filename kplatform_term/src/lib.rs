use std::io::{self, Write};
use std::time::Duration;

use kplatform_core::{Event, Platform, PlatformError, WindowConfig};

/// Minimal terminal-backed platform. Renders using ANSI truecolor escapes.
#[derive(Clone, Debug)]
pub struct TermPlatform {
    size: (u32, u32),
}
impl TermPlatform {
    pub fn new(config: &WindowConfig) -> Result<Self, PlatformError> {
        let size = (config.width, config.height);
        Ok(Self { size })
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
            for &px in row {
                let [r, g, b, _a] = px.to_le_bytes();
                // Truecolor background + one space cell
                // ESC[48;2;R;G;Bm  (background color), then space character
                // We do not optimize run lengths in this first cut.
                write!(w, "\x1b[48;2;{};{};{}m ", r, g, b)?;
            }
            // Reset attributes and newline per row
            w.write_all(b"\x1b[0m\n")?;
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
}
