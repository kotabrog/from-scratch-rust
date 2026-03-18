use std::error::Error;
use std::fmt;
use std::io;
use std::time::Duration;

/// Window configuration for creating a platform surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WindowConfig {
    /// Window title or backend-specific label.
    pub title: String,
    /// Logical drawable width in pixels/cells.
    pub width: u32,
    /// Logical drawable height in pixels/cells.
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
    /// User requested the window/platform session to close.
    CloseRequested,
    /// Key press event.
    KeyDown(Key),
    /// Key release event.
    KeyUp(Key),
    /// Pointer moved in logical coordinates.
    MouseMove { x: i32, y: i32 },
    /// Mouse button press in logical coordinates.
    MouseDown { button: MouseButton, x: i32, y: i32 },
    /// Mouse button release in logical coordinates.
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
    ///
    /// Backends without resize support may keep returning the size provided at
    /// creation time.
    fn size(&self) -> (u32, u32);

    /// Polls one pending event without blocking.
    ///
    /// Returns `Some(Event)` when an event is available, otherwise `None`.
    /// Backends should preserve the order in which events were received.
    fn poll_event(&mut self) -> Option<Event>;

    /// Presents an RGBA little-endian pixel buffer of size `(width, height)`.
    ///
    /// `pixels_rgba_le` must contain at least `width * height` pixels.
    /// Each pixel is interpreted as `u32::from_le_bytes([r, g, b, a])`.
    /// Backends may ignore alpha when the target does not support blending.
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
    fn key_variants_constructible() {
        assert_eq!(Key::Char('a'), Key::Char('a'));
        assert_eq!(Key::Escape, Key::Escape);
        assert_eq!(Key::Left, Key::Left);
    }

    #[test]
    fn mouse_button_variants_constructible() {
        assert_eq!(MouseButton::Left, MouseButton::Left);
        assert_eq!(MouseButton::Other(4), MouseButton::Other(4));
    }

    #[test]
    fn event_variants_constructible() {
        let _e1 = Event::CloseRequested;
        let _e2 = Event::KeyDown(Key::Escape);
        let _e3 = Event::KeyUp(Key::Enter);
        let _e4 = Event::MouseMove { x: 3, y: 4 };
        let _e5 = Event::MouseDown {
            button: MouseButton::Left,
            x: 1,
            y: 2,
        };
        let _e6 = Event::MouseUp {
            button: MouseButton::Right,
            x: 5,
            y: 6,
        };
    }

    #[test]
    fn platform_error_display_variants() {
        assert_eq!(
            format!("{}", PlatformError::Unsupported),
            "Unsupported platform or configuration"
        );
        assert_eq!(
            format!("{}", PlatformError::Backend("x11 failed".to_string())),
            "Backend error: x11 failed"
        );
    }

    #[test]
    fn platform_error_from_io() {
        let e = io::Error::other("boom");
        let pe: PlatformError = e.into();
        assert!(format!("{}", pe).contains("IO error"));
    }
}
