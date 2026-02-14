use std::time::{Duration, Instant};

use kdev::out;
use kloop::{App, FixedLoop, LoopConfig};
use kpix::{Color, Surface};
use std::process::Command;

struct BallDemo {
    // physics state (curr/prev)
    px: f32,
    py: f32,
    vx: f32,
    vy: f32,
    prev_px: f32,
    prev_py: f32,
    w: usize,
    h: usize,
}

impl BallDemo {
    fn new(w: usize, h: usize) -> Self {
        Self {
            px: 40.0,
            py: 40.0,
            vx: 60.0,
            vy: 85.0,
            prev_px: 40.0,
            prev_py: 40.0,
            w,
            h,
        }
    }

    fn draw(&self, s: &mut Surface, alpha: f32) {
        // background
        s.clear(Color::rgba(20, 30, 50, 255));
        // interpolate position for rendering
        let x = self.prev_px + (self.px - self.prev_px) * alpha;
        let y = self.prev_py + (self.py - self.prev_py) * alpha;
        // simple filled circle (approx) using set_pixel
        let cx = x as i32;
        let cy = y as i32;
        let r = 8i32;
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    s.set_pixel(cx + dx, cy + dy, Color::rgba(230, 180, 40, 255));
                }
            }
        }
    }
}

impl App for BallDemo {
    fn update(&mut self, dt: Duration) {
        // save previous state for interpolation
        self.prev_px = self.px;
        self.prev_py = self.py;
        let dt_s = dt.as_secs_f32();
        self.px += self.vx * dt_s;
        self.py += self.vy * dt_s;
        // bounce on walls
        if self.px < 8.0 {
            self.px = 8.0;
            self.vx = self.vx.abs();
        }
        if self.py < 8.0 {
            self.py = 8.0;
            self.vy = self.vy.abs();
        }
        if self.px > (self.w as f32 - 8.0) {
            self.px = self.w as f32 - 8.0;
            self.vx = -self.vx.abs();
        }
        if self.py > (self.h as f32 - 8.0) {
            self.py = self.h as f32 - 8.0;
            self.vy = -self.vy.abs();
        }
    }

    fn render(&mut self, _alpha: f32) {
        // handled explicitly in main to write PPM each frame
    }
}

fn main() {
    // Option parsing: --video to encode MP4, --realtime <seconds> to run with real time
    let mut make_video = false;
    let mut realtime_secs: Option<f64> = None;
    let mut args = std::env::args().skip(1).peekable();
    while let Some(a) = args.next() {
        if a == "--video" {
            make_video = true;
        } else if a == "--realtime" {
            if let Some(s) = args.next() {
                match s.parse::<f64>() {
                    Ok(v) if v > 0.0 => realtime_secs = Some(v),
                    _ => {
                        eprintln!("--realtime の秒数は正の数で指定してください（例: --realtime 2.0）");
                        return;
                    }
                }
            } else {
                eprintln!("--realtime の後に秒数を指定してください（例: --realtime 2.0）");
                return;
            }
        }
    }

    let (w, h) = (256usize, 256usize);
    let mut surface = Surface::new(w, h);

    let mut app = BallDemo::new(w, h);
    let out_dir = out::example_output_dir("kloop_demo").expect("create out dir");

    if let Some(secs) = realtime_secs {
        // Real-time mode: use SystemClock and tick until specified seconds elapse.
        let cfg = LoopConfig::from_hz(60).with_limits(Duration::from_millis(250), 1000);
        let mut looper = FixedLoop::new(ktime::SystemClock, cfg);

        let start = Instant::now();
        let target_frame = Duration::from_secs_f64(1.0 / 60.0);
        let mut last_frame = Instant::now();
        let mut i = 0u32;
        while start.elapsed().as_secs_f64() < secs {
            let res = looper.tick(&mut app);
            app.draw(&mut surface, res.alpha);
            let path = out_dir.join(format!("frame_{:06}.ppm", i));
            kpix::io::write_ppm(&surface, path).expect("write ppm");
            i += 1;

            // Pace roughly to 60 FPS
            let elapsed = last_frame.elapsed();
            if elapsed < target_frame {
                std::thread::sleep(target_frame - elapsed);
            }
            last_frame = Instant::now();
        }
    } else {
        // Default: headless deterministic capture using FakeClock for exactly 120 frames (2 seconds @ 60fps)
        let clock = ktime::FakeClock::default();
        let cfg = LoopConfig::from_hz(60).with_limits(Duration::from_millis(250), 1000);
        let mut looper = FixedLoop::new(clock, cfg);

        for i in 0..120u32 {
            // Advance by one fixed step deterministically
            looper.run_steps(&mut app, 1);
            // Since run_steps doesn't call render, synthesize alpha=0 and draw now
            app.draw(&mut surface, 0.0);
            let path = out_dir.join(format!("frame_{:06}.ppm", i));
            kpix::io::write_ppm(&surface, path).expect("write ppm");
        }
    }

    // Optional: create a video from frames using ffmpeg when --video is passed.
    if make_video {
        println!(
            "Encoding out.mp4 via ffmpeg in {:?} (60 fps)",
            out_dir
        );
        let status = Command::new("ffmpeg")
            .args([
                "-y",
                "-framerate",
                "60",
                "-i",
                "frame_%06d.ppm",
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                "out.mp4",
            ])
            .current_dir(&out_dir)
            .status();
        match status {
            Ok(s) if s.success() => {
                println!("Created {:?}/out.mp4", out_dir);
            }
            Ok(s) => {
                eprintln!("ffmpeg exited with status: {:?}", s.code());
            }
            Err(e) => {
                eprintln!("Failed to run ffmpeg: {}", e);
            }
        }
    }
}
