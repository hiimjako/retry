pub mod pretty_print {
    use std::time::Duration;
    pub fn duration(duration: Duration) -> String {
        let millis = duration.as_millis();
        if millis == 0 {
            return "0ms".to_string();
        }

        let days = millis / (1000 * 60 * 60 * 24);
        let remaining_millis = millis % (1000 * 60 * 60 * 24);

        let hours = remaining_millis / (1000 * 60 * 60);
        let remaining_millis = remaining_millis % (1000 * 60 * 60);

        let minutes = remaining_millis / (1000 * 60);
        let remaining_millis = remaining_millis % (1000 * 60);

        let seconds = remaining_millis / 1000;
        let millis = remaining_millis % 1000;

        let mut parts = Vec::new();

        if days > 0 {
            parts.push(format!("{}d", days));
        }

        if hours > 0 {
            parts.push(format!("{}h", hours));
        }

        if minutes > 0 {
            parts.push(format!("{}m", minutes));
        }

        if seconds > 0 {
            parts.push(format!("{}s", seconds));
        }

        if millis > 0 || parts.is_empty() {
            // Add milliseconds if they exist or if we have no larger units
            if millis > 0 {
                parts.push(format!("{}ms", millis));
            } else {
                parts.push("0ms".to_string());
            }
        }

        parts.join("")
    }
}
