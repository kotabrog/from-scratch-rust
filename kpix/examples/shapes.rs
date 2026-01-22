use kpix::{Color, Surface, io};

fn main() {
    let (w, h) = (256i32, 256i32);
    let mut s = Surface::new(w as usize, h as usize);

    // Background
    s.clear(Color::rgba(20, 20, 30, 255));

    // Grid lines every 32 px
    let grid = Color::rgba(60, 60, 80, 255);
    for y in (0..h).step_by(32) {
        kpix::draw::draw_line(&mut s, 0, y, w - 1, y, grid);
    }
    for x in (0..w).step_by(32) {
        kpix::draw::draw_line(&mut s, x, 0, x, h - 1, grid);
    }

    // Star from center
    let cx = w / 2;
    let cy = h / 2;
    let star = Color::rgba(230, 180, 30, 255);
    let corners = [
        (0, 0),
        (w - 1, 0),
        (w - 1, h - 1),
        (0, h - 1),
        (w - 1, cy),
        (0, cy),
        (cx, 0),
        (cx, h - 1),
    ];
    for &(x, y) in &corners {
        kpix::draw::draw_line(&mut s, cx, cy, x, y, star);
    }

    io::write_ppm(&s, "shapes.ppm").expect("failed to write PPM");
}
