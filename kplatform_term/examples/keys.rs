use std::time::{Duration, Instant};

use kplatform_core::{Event, Platform, WindowConfig};
use kplatform_term::TermPlatform;

fn main() {
    // Logical surface size is not used here (no drawing), but we provide a small size.
    let cfg = WindowConfig::new("term-keys", 40, 10);
    let mut plat = TermPlatform::new(&cfg).expect("term platform");

    // Optional seconds arg: --seconds <f32>
    let mut seconds: f32 = 10.0;
    let mut args = std::env::args().skip(1);
    if let (Some(a), Some(s)) = (args.next(), args.next())
        && a == "--seconds"
        && let Ok(sec) = s.parse::<f32>()
        && sec > 0.0
    {
        seconds = sec;
    }

    let start = Instant::now();
    let mut events: Vec<String> = Vec::new();

    loop {
        // Drain all pending events this tick
        while let Some(ev) = plat.poll_event() {
            match ev {
                Event::KeyDown(k) => {
                    let s = match k {
                        kplatform_core::Key::Escape => "KeyDown(Escape)".to_string(),
                        kplatform_core::Key::Enter => "KeyDown(Enter)".to_string(),
                        kplatform_core::Key::Backspace => "KeyDown(Backspace)".to_string(),
                        kplatform_core::Key::Left => "KeyDown(Left)".to_string(),
                        kplatform_core::Key::Right => "KeyDown(Right)".to_string(),
                        kplatform_core::Key::Up => "KeyDown(Up)".to_string(),
                        kplatform_core::Key::Down => "KeyDown(Down)".to_string(),
                        kplatform_core::Key::Char(c) => format!("KeyDown(Char('{}'))", c),
                    };
                    events.push(s);
                }
                Event::KeyUp(k) => {
                    let s = match k {
                        kplatform_core::Key::Escape => "KeyUp(Escape)".to_string(),
                        kplatform_core::Key::Enter => "KeyUp(Enter)".to_string(),
                        kplatform_core::Key::Backspace => "KeyUp(Backspace)".to_string(),
                        kplatform_core::Key::Left => "KeyUp(Left)".to_string(),
                        kplatform_core::Key::Right => "KeyUp(Right)".to_string(),
                        kplatform_core::Key::Up => "KeyUp(Up)".to_string(),
                        kplatform_core::Key::Down => "KeyUp(Down)".to_string(),
                        kplatform_core::Key::Char(c) => format!("KeyUp(Char('{}'))", c),
                    };
                    events.push(s);
                }
                Event::MouseMove { .. } => events.push("MouseMove".to_string()),
                Event::MouseDown { .. } => events.push("MouseDown".to_string()),
                Event::MouseUp { .. } => events.push("MouseUp".to_string()),
                Event::CloseRequested => {
                    events.push("CloseRequested".to_string());
                    break;
                }
            }
        }

        // Quit when 'q' is pressed, or timeout exceeded
        if events.iter().any(|s| s.contains("Char('q')")) {
            break;
        }
        if start.elapsed().as_secs_f32() >= seconds {
            break;
        }

        plat.sleep(Duration::from_millis(16));
    }

    // Leave alternate screen before printing captured events
    drop(plat);
    println!(
        "Captured {} events (q to quit, or timed out):",
        events.len()
    );
    for (i, e) in events.iter().enumerate() {
        println!("{:04}: {}", i, e);
    }
}
