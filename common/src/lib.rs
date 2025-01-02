use std::time::{Duration, Instant};

pub struct Timed<T> {
    start: Instant,
    end: Instant,
    pub output: T,
}

impl<T> Timed<T> {
    pub fn elapsed(&self) -> Duration {
        self.end.duration_since(self.start)
    }
}

pub fn time<F, T>(func: F) -> Timed<T>
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let output = func();
    let end = Instant::now();
    Timed { start, end, output }
}
