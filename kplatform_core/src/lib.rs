use std::error::Error;
use std::fmt;
use std::io;
use std::time::Duration;

/// Window configuration for creating a platform surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl WindowConfig {
    pub fn new(title: impl Into<String>, width: u32, height: u32) -> Self {
        Self {
            title: title.into(),
            width,
            height,
        }
    }
}

/// Keyboard key representation (minimal set for MVP).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Key {
    Escape,
    Enter,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Char(char),
}

/// Mouse button representation.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u8),
}

/// Platform-agnostic input events.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    CloseRequested,
    KeyDown(Key),
    KeyUp(Key),
    MouseMove { x: i32, y: i32 },
    MouseDown { button: MouseButton, x: i32, y: i32 },
    MouseUp { button: MouseButton, x: i32, y: i32 },
}

/// Unified platform error type.
#[derive(Debug)]
pub enum PlatformError {
    Io(io::Error),
    Unsupported,
    Backend(String),
}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlatformError::Io(e) => write!(f, "IO error: {}", e),
            PlatformError::Unsupported => write!(f, "Unsupported platform or configuration"),
            PlatformError::Backend(s) => write!(f, "Backend error: {}", s),
        }
    }
}

impl Error for PlatformError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PlatformError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for PlatformError {
    fn from(e: io::Error) -> Self {
        PlatformError::Io(e)
    }
}

/// Minimal platform trait implemented by backends (terminal, X11, etc.).
pub trait Platform {
    /// Returns the logical size of the drawable area in pixels/cells.
    fn size(&self) -> (u32, u32);

    /// Non-blocking event poll. Returns `Some(Event)` if available.
    fn poll_event(&mut self) -> Option<Event>;

    /// Present an RGBA little-endian pixel buffer of size (width, height).
    /// The buffer length must be at least width*height.
    fn present_rgba_le(
        &mut self,
        width: u32,
        height: u32,
        pixels_rgba_le: &[u32],
    ) -> Result<(), PlatformError>;

    /// Sleep the current thread for `dt`.
    fn sleep(&self, dt: Duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_config_new() {
        let cfg = WindowConfig::new("hello", 320, 240);
        assert_eq!(cfg.title, "hello");
        assert_eq!(cfg.width, 320);
        assert_eq!(cfg.height, 240);
    }

    #[test]
    fn platform_error_from_io() {
        let e = io::Error::other("boom");
        let pe: PlatformError = e.into();
        assert!(format!("{}", pe).contains("IO error"));
    }

    #[test]
    fn event_variants_constructible() {
        let _e1 = Event::CloseRequested;
        let _e2 = Event::KeyDown(Key::Escape);
        let _e3 = Event::MouseDown {
            button: MouseButton::Left,
            x: 1,
            y: 2,
        };
    }
}
