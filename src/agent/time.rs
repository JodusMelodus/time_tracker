use std::time::{Duration, Instant};

pub struct StopWatch {
    start: Option<Instant>,
    elapsed: Duration,
}

impl StopWatch {
    pub fn new() -> Self {
        Self {
            start: None,
            elapsed: Duration::ZERO,
        }
    }

    pub fn start(&mut self) {
        if self.start.is_none() {
            self.start = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(s) = self.start.take() {
            self.elapsed += s.elapsed();
        }
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.elapsed = Duration::ZERO;
    }

    pub fn elapsed(&mut self) -> Duration {
        match self.start {
            Some(s) => self.elapsed + s.elapsed(),
            None => self.elapsed,
        }
    }
}

