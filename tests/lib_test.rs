#[cfg(test)]
mod tests {
    use retry::pretty_print;

    use std::time::Duration; // Assuming your function is in the root of your crate or modify the path

    #[test]
    fn test_zero_duration() {
        assert_eq!(pretty_print::duration(Duration::from_secs(0)), "0ms");
    }

    #[test]
    fn test_seconds_only() {
        assert_eq!(pretty_print::duration(Duration::from_secs(5)), "5s");
    }

    #[test]
    fn test_minutes_and_seconds() {
        assert_eq!(pretty_print::duration(Duration::from_secs(65)), "1m5s");
    }

    #[test]
    fn test_hours_minutes_seconds() {
        assert_eq!(pretty_print::duration(Duration::from_secs(3665)), "1h1m5s");
    }

    #[test]
    fn test_days_hours_minutes_seconds() {
        assert_eq!(
            pretty_print::duration(Duration::from_secs(86400 + 3600 + 60 + 10)),
            "1d1h1m10s"
        );
    }

    #[test]
    fn test_milliseconds() {
        assert_eq!(
            pretty_print::duration(Duration::from_millis(5500)),
            "5s500ms"
        );
    }

    #[test]
    fn test_sub_second_with_seconds() {
        assert_eq!(
            pretty_print::duration(Duration::from_secs(2) + Duration::from_nanos(500_000_000)),
            "2s500ms"
        );
    }
}
