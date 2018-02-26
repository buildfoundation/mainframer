use std::fmt::Write;
use std::time::Duration;

pub fn format_duration(duration: &Duration) -> String {
    let total_seconds = duration.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

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
        push_value_and_label(&mut text, hours, hours_label);
    }

    if minutes > 0 {
        if !text.is_empty() {
            text.push(' ');
        }

        push_value_and_label(&mut text, minutes, minutes_label);
    }

    if !text.is_empty() {
        text.push(' ');
    }

    push_value_and_label(&mut text, seconds, seconds_label);

    text
}

fn push_value_and_label(text: &mut String, value: u64, label: &str) {
    write!(text, "{} {}", value, label).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_0_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(0)), "0 seconds");
    }

    #[test]
    fn format_duration_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(1)), "1 second");
    }

    #[test]
    fn format_duration_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(2)), "2 seconds");
    }

    #[test]
    fn format_duration_1_minute() {
        assert_eq!(format_duration(&Duration::from_secs(60)), "1 minute 0 seconds");    }

    #[test]
    fn format_duration_1_minute_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(61)), "1 minute 1 second");    }

    #[test]
    fn format_duration_1_minute_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(62)), "1 minute 2 seconds");    }

    #[test]
    fn format_duration_2_minutes() {
        assert_eq!(format_duration(&Duration::from_secs(120)), "2 minutes 0 seconds");    }

    #[test]
    fn format_duration_2_minutes_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(121)), "2 minutes 1 second");    }

    #[test]
    fn format_duration_2_minutes_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(122)), "2 minutes 2 seconds");    }

    #[test]
    fn format_duration_1_hour() {
        assert_eq!(format_duration(&Duration::from_secs(3600)), "1 hour 0 seconds");    }

    #[test]
    fn format_duration_1_hour_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(3601)), "1 hour 1 second");    }

    #[test]
    fn format_duration_1_hour_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(3602)), "1 hour 2 seconds");    }

    #[test]
    fn format_duration_1_hour_1_minute() {
        assert_eq!(format_duration(&Duration::from_secs(3660)), "1 hour 1 minute 0 seconds");
    }

    #[test]
    fn format_duration_1_hour_1_minute_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(3661)), "1 hour 1 minute 1 second");
    }

    #[test]
    fn format_duration_1_hour_1_minute_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(3662)), "1 hour 1 minute 2 seconds");
    }

    #[test]
    fn format_duration_1_hour_2_minutes() {
        assert_eq!(format_duration(&Duration::from_secs(3720)), "1 hour 2 minutes 0 seconds");
    }

    #[test]
    fn format_duration_1_hour_2_minutes_1_second() {
        assert_eq!(format_duration(&Duration::from_secs(3721)), "1 hour 2 minutes 1 second");
    }

    #[test]
    fn format_duration_1_hour_2_minutes_2_seconds() {
        assert_eq!(format_duration(&Duration::from_secs(3722)), "1 hour 2 minutes 2 seconds");
    }
}
