use std::time::Duration;

pub trait App {
    fn update(&mut self, dt: Duration);
    fn render(&mut self, _alpha: f32) {
        let _ = _alpha;
    }
}
