use crate::utils::is_logging_enabled;
use log::{debug, error, warn};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct GameTimer {
    start_time: Option<Instant>,
    elapsed: Duration,
    is_running: bool,
}

impl GameTimer {
    pub fn new() -> Self {
        if is_logging_enabled() {
            debug!("Creating new GameTimer");
        }
        Self {
            start_time: None,
            elapsed: Duration::ZERO,
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        if !self.is_running {
            if is_logging_enabled() {
                debug!("Starting timer");
            }
            self.start_time = Some(Instant::now());
            self.is_running = true;
        } else if is_logging_enabled() {
            warn!("Attempted to start timer that is already running");
        }
    }

    pub fn pause(&mut self) {
        if self.is_running {
            if is_logging_enabled() {
                debug!("Pausing timer");
            }
            if let Some(start) = self.start_time {
                let current_elapsed = start.elapsed();
                self.elapsed += current_elapsed;
                if is_logging_enabled() {
                    debug!("Timer paused after {:?} additional time", current_elapsed);
                }
            } else if is_logging_enabled() {
                error!("Timer was running but start_time was None - this should not happen!");
            }
            self.is_running = false;
        } else if is_logging_enabled() {
            warn!("Attempted to pause timer that is not running");
        }
    }

    pub fn reset(&mut self) {
        if is_logging_enabled() {
            debug!("Resetting timer");
        }
        self.start_time = None;
        self.elapsed = Duration::ZERO;
        self.is_running = false;
    }

    pub fn get_elapsed(&self) -> Duration {
        let mut total = self.elapsed;
        if self.is_running {
            if let Some(start) = self.start_time {
                total += start.elapsed();
            } else {
                error!("Timer is running but start_time is None - this should not happen!");
            }
        }
        total
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    #[allow(dead_code)]
    pub fn debug_state(&self) -> String {
        format!(
            "Timer state: running={}, elapsed={:?}, start_time={:?}",
            self.is_running,
            self.elapsed,
            self.start_time.map(|t| t.elapsed())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_timer_initialization() {
        let timer = GameTimer::new();

        assert!(!timer.is_running());
        assert_eq!(timer.get_elapsed(), Duration::ZERO);
    }

    #[test]
    fn test_timer_start() {
        let mut timer = GameTimer::new();

        assert!(!timer.is_running());
        timer.start();
        assert!(timer.is_running());
    }

    #[test]
    fn test_timer_start_already_running() {
        let mut timer = GameTimer::new();

        timer.start();
        assert!(timer.is_running());

        timer.start();
        assert!(timer.is_running());
    }

    #[test]
    fn test_timer_pause() {
        let mut timer = GameTimer::new();

        timer.start();
        assert!(timer.is_running());

        timer.pause();
        assert!(!timer.is_running());
    }

    #[test]
    fn test_timer_pause_not_running() {
        let mut timer = GameTimer::new();

        assert!(!timer.is_running());
        timer.pause();
        assert!(!timer.is_running());
    }

    #[test]
    fn test_timer_reset() {
        let mut timer = GameTimer::new();

        timer.start();
        assert!(timer.is_running());

        timer.reset();
        assert!(!timer.is_running());
        assert_eq!(timer.get_elapsed(), Duration::ZERO);
    }

    #[test]
    fn test_timer_elapsed_while_running() {
        let mut timer = GameTimer::new();

        timer.start();

        thread::sleep(Duration::from_millis(10));

        let elapsed = timer.get_elapsed();
        assert!(elapsed >= Duration::from_millis(10));
        assert!(elapsed < Duration::from_millis(100));
    }

    #[test]
    fn test_timer_elapsed_after_pause() {
        let mut timer = GameTimer::new();

        timer.start();
        thread::sleep(Duration::from_millis(10));
        timer.pause();

        let elapsed_after_pause = timer.get_elapsed();
        assert!(elapsed_after_pause >= Duration::from_millis(10));

        thread::sleep(Duration::from_millis(10));
        let elapsed_after_sleep = timer.get_elapsed();
        assert_eq!(elapsed_after_pause, elapsed_after_sleep);
    }

    #[test]
    fn test_timer_resume_after_pause() {
        let mut timer = GameTimer::new();

        timer.start();
        thread::sleep(Duration::from_millis(10));
        timer.pause();
        let elapsed_after_pause = timer.get_elapsed();

        timer.start();
        thread::sleep(Duration::from_millis(10));

        let elapsed_after_resume = timer.get_elapsed();
        assert!(elapsed_after_resume > elapsed_after_pause);
    }

    #[test]
    fn test_timer_multiple_pause_resume_cycles() {
        let mut timer = GameTimer::new();

        timer.start();
        thread::sleep(Duration::from_millis(5));
        timer.pause();
        let first_elapsed = timer.get_elapsed();

        timer.start();
        thread::sleep(Duration::from_millis(5));
        timer.pause();
        let second_elapsed = timer.get_elapsed();

        assert!(second_elapsed > first_elapsed);
        assert!(second_elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_timer_reset_clears_elapsed() {
        let mut timer = GameTimer::new();

        timer.start();
        thread::sleep(Duration::from_millis(10));
        timer.pause();

        let elapsed_before_reset = timer.get_elapsed();
        assert!(elapsed_before_reset > Duration::ZERO);

        timer.reset();
        assert_eq!(timer.get_elapsed(), Duration::ZERO);
        assert!(!timer.is_running());
    }

    #[test]
    fn test_timer_debug_state() {
        let mut timer = GameTimer::new();

        let debug_str = timer.debug_state();
        assert!(debug_str.contains("running=false"));
        assert!(debug_str.contains("elapsed=0"));

        timer.start();
        let debug_str_running = timer.debug_state();
        assert!(debug_str_running.contains("running=true"));
    }

    #[test]
    fn test_timer_clone() {
        let mut timer = GameTimer::new();
        timer.start();
        thread::sleep(Duration::from_millis(5));

        let cloned = timer.clone();
        assert_eq!(timer.is_running(), cloned.is_running());
        let time_diff =
            timer.get_elapsed().as_millis() as i64 - cloned.get_elapsed().as_millis() as i64;
        assert!(time_diff.abs() < 10);
    }

    #[test]
    fn test_timer_debug_formatting() {
        let timer = GameTimer::new();
        let debug_str = format!("{:?}", timer);
        assert!(debug_str.contains("GameTimer"));
    }
}
