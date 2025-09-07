use std::time::Duration;

pub fn format_time(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time_zero() {
        let duration = Duration::from_secs(0);
        assert_eq!(format_time(duration), "00:00");
    }

    #[test]
    fn test_format_time_seconds_only() {
        let duration = Duration::from_secs(30);
        assert_eq!(format_time(duration), "00:30");
    }

    #[test]
    fn test_format_time_minutes_only() {
        let duration = Duration::from_secs(120);
        assert_eq!(format_time(duration), "02:00");
    }

    #[test]
    fn test_format_time_minutes_and_seconds() {
        let duration = Duration::from_secs(125);
        assert_eq!(format_time(duration), "02:05");
    }

    #[test]
    fn test_format_time_large_values() {
        let duration = Duration::from_secs(3661);
        assert_eq!(format_time(duration), "61:01");
    }

    #[test]
    fn test_format_time_max_seconds() {
        let duration = Duration::from_secs(3599);
        assert_eq!(format_time(duration), "59:59");
    }

    #[test]
    fn test_format_time_with_milliseconds() {
        let duration = Duration::from_millis(125000);
        assert_eq!(format_time(duration), "02:05");
    }

    #[test]
    fn test_format_time_edge_cases() {
        assert_eq!(format_time(Duration::from_secs(1)), "00:01");
        assert_eq!(format_time(Duration::from_secs(60)), "01:00");
        assert_eq!(format_time(Duration::from_secs(61)), "01:01");
        assert_eq!(format_time(Duration::from_secs(999)), "16:39");
    }
}
