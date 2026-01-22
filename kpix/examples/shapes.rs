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

    // Rectangles: outlines and filled
    let outline = Color::rgba(200, 80, 80, 255);
    let fill = Color::rgba(80, 160, 200, 255);
    kpix::draw::draw_rect(&mut s, 20, 20, 60, 40, outline);
    kpix::draw::fill_rect(&mut s, 24, 24, 52, 32, fill);
    // Negative size normalization
    kpix::draw::draw_rect(&mut s, 200, 200, -40, -30, outline);
    kpix::draw::fill_rect(&mut s, 196, 196, -32, -22, fill);

    io::write_ppm(&s, "shapes.ppm").expect("failed to write PPM");
}
