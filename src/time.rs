use std::time::Duration;

pub fn format_duration(duration: &Duration) -> String {
    let raw_seconds = duration.as_secs();

    let hours = raw_seconds / 3600;
    let minutes = (raw_seconds % 3600) / 60;
    let seconds = raw_seconds % 60;

    let hours_label = match hours {
        1 => "hour",
        _ => "hours"
    };

    let minutes_label = match minutes {
        1 => "minute",
        _ => "minutes"
    };

    let seconds_label = match seconds {
        1 => "second",
        _ => "seconds"
    };

    let mut text = String::new();

    if hours > 0 {
        text.push_str(&format!("{value} {label}", value = hours, label = hours_label));
    }

    if minutes > 0 {
        if !text.is_empty() {
            text.push(' ');
        }

        text.push_str(&format!("{value} {label}", value = minutes, label = minutes_label));
    }

    if !text.is_empty() {
        text.push(' ');
    }

    text.push_str(&format!("{value} {label}", value = seconds, label = seconds_label));

    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_0_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(0)), String::from("0 seconds"));
    }

    #[test]
    fn format_duration_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(1)), String::from("1 second"));
    }

    #[test]
    fn format_duration_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(2)), String::from("2 seconds"));
    }

    #[test]
    fn format_duration_1_minute() {
        assert_eq!(format_duration(&Duration::from_secs(60)), String::from("1 minute 0 seconds"));
    }

    #[test]
    fn format_duration_1_minute_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(61)), String::from("1 minute 1 second"));
    }

    #[test]
    fn format_duration_1_minute_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(62)), String::from("1 minute 2 seconds"));
    }

    #[test]
    fn format_duration_2_minutes() {
        assert_eq!(format_duration(&Duration::from_secs(120)), String::from("2 minutes 0 seconds"));
    }

    #[test]
    fn format_duration_2_minutes_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(121)), String::from("2 minutes 1 second"));
    }

    #[test]
    fn format_duration_2_minutes_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(122)), String::from("2 minutes 2 seconds"));
    }

    #[test]
    fn format_duration_1_hour() {
        assert_eq!(format_duration(&Duration::from_secs(3600)), String::from("1 hour 0 seconds"));
    }

    #[test]
    fn format_duration_1_hour_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(3601)), String::from("1 hour 1 second"));
    }

    #[test]
    fn format_duration_1_hour_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(3602)), String::from("1 hour 2 seconds"));
    }

    #[test]
    fn format_duration_1_hour_1_minute() {
        assert_eq!(format_duration(&Duration::from_secs(3660)), String::from("1 hour 1 minute 0 seconds"));
    }

    #[test]
    fn format_duration_1_hour_1_minute_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(3661)), String::from("1 hour 1 minute 1 second"));
    }

    #[test]
    fn format_duration_1_hour_1_minute_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(3662)), String::from("1 hour 1 minute 2 seconds"));
    }

    #[test]
    fn format_duration_1_hour_2_minutes() {
        assert_eq!(format_duration(&Duration::from_secs(3720)), String::from("1 hour 2 minutes 0 seconds"));
    }

    #[test]
    fn format_duration_1_hour_2_minutes_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(3721)), String::from("1 hour 2 minutes 1 second"));
    }

    #[test]
    fn format_duration_1_hour_2_minutes_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(3722)), String::from("1 hour 2 minutes 2 seconds"));
    }
}
