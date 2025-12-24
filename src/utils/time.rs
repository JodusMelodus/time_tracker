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

pub fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();

    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
