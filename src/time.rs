use std::time::Duration;

pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let millis = (duration.as_millis() - (u128::from(total_seconds) * 1000)) % 1000;

    if hours > 0 {
        format!("{} {} {} {}", hours, hours_label(hours), minutes, minutes_label(minutes))
    } else if minutes > 0 {
        format!("{} {} {} {}", minutes, minutes_label(minutes), seconds, seconds_label(minutes, seconds))
    } else {
        format!("{}.{:03} {}", seconds, millis, seconds_label(minutes, seconds))
    }
}

fn hours_label(hours: u64) -> String {
    match hours {
        1 => "hour",
        _ => "hours"
    }.to_owned()
}

fn minutes_label(minutes: u64) -> String {
    match minutes {
        1 => "minute",
        _ => "minutes"
    }.to_owned()
}

fn seconds_label(minutes: u64, seconds: u64) -> String {
    if minutes > 0 {
        match seconds {
            1 => "second",
            _ => "seconds"
        }
    } else {
        "seconds"
    }.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_duration_0_seconds() {
        assert_eq!(format_duration(Duration::from_secs(0)), "0.000 seconds");
    }

    #[test]
    fn format_duration_0_5_seconds() {
        assert_eq!(format_duration(Duration::from_millis(500)), "0.500 seconds");
    }

    #[test]
    fn format_duration_1_second() {
        assert_eq!(format_duration(Duration::from_secs(1)), "1.000 seconds");
    }

    #[test]
    fn format_duration_1_5_second() {
        assert_eq!(format_duration(Duration::from_millis(1500)), "1.500 seconds");
    }

    #[test]
    fn format_duration_2_seconds() {
        assert_eq!(format_duration(Duration::from_secs(2)), "2.000 seconds");
    }

    #[test]
    fn format_duration_1_minute() {
        assert_eq!(format_duration(Duration::from_secs(60)), "1 minute 0 seconds");
    }

    #[test]
    fn format_duration_1_minute_1_second() {
        assert_eq!(format_duration(Duration::from_secs(61)), "1 minute 1 second");
    }

    #[test]
    fn format_duration_1_minute_1_5_second() {
        assert_eq!(format_duration(Duration::from_millis(61500)), "1 minute 1 second");
    }

    #[test]
    fn format_duration_1_minute_2_seconds() {
        assert_eq!(format_duration(Duration::from_secs(62)), "1 minute 2 seconds");
    }

    #[test]
    fn format_duration_2_minutes() {
        assert_eq!(format_duration(Duration::from_secs(120)), "2 minutes 0 seconds");
    }

    #[test]
    fn format_duration_2_minutes_1_second() {
        assert_eq!(format_duration(Duration::from_secs(121)), "2 minutes 1 second");
    }

    #[test]
    fn format_duration_2_minutes_2_seconds() {
        assert_eq!(format_duration(Duration::from_secs(122)), "2 minutes 2 seconds");
    }

    #[test]
    fn format_duration_1_hour() {
        assert_eq!(format_duration(Duration::from_secs(3600)), "1 hour 0 minutes");
    }

    #[test]
    fn format_duration_1_hour_1_second() {
        assert_eq!(format_duration(Duration::from_secs(3601)), "1 hour 0 minutes");
    }

    #[test]
    fn format_duration_1_hour_1_5_second() {
        assert_eq!(format_duration(Duration::from_millis(3601 * 1000 + 500)), "1 hour 0 minutes");
    }

    #[test]
    fn format_duration_1_hour_2_seconds() {
        assert_eq!(format_duration(Duration::from_secs(3602)), "1 hour 0 minutes");
    }

    #[test]
    fn format_duration_1_hour_1_minute() {
        assert_eq!(format_duration(Duration::from_secs(3660)), "1 hour 1 minute");
    }

    #[test]
    fn format_duration_1_hour_1_minute_1_second() {
        assert_eq!(format_duration(Duration::from_secs(3661)), "1 hour 1 minute");
    }

    #[test]
    fn format_duration_1_hour_1_minute_1_5_second() {
        assert_eq!(format_duration(Duration::from_millis(3661 * 1000 + 500)), "1 hour 1 minute");
    }

    #[test]
    fn format_duration_1_hour_1_minute_2_seconds() {
        assert_eq!(format_duration(Duration::from_secs(3662)), "1 hour 1 minute");
    }

    #[test]
    fn format_duration_1_hour_2_minutes() {
        assert_eq!(format_duration(Duration::from_secs(3720)), "1 hour 2 minutes");
    }

    #[test]
    fn format_duration_1_hour_2_minutes_1_second() {
        assert_eq!(format_duration(Duration::from_secs(3721)), "1 hour 2 minutes");
    }

    #[test]
    fn format_duration_1_hour_2_minutes_2_seconds() {
        assert_eq!(format_duration(Duration::from_secs(3722)), "1 hour 2 minutes");
    }
}
