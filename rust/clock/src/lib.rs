use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours.rem_euclid(HOURS_PER_DAY) * MINUTES_PER_HOUR)
            + minutes.rem_euclid(HOURS_PER_DAY * MINUTES_PER_HOUR);

        Clock {
            minutes: total_minutes.rem_euclid(HOURS_PER_DAY * MINUTES_PER_HOUR),
        }
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;

        self
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minutes = self.minutes.rem_euclid(MINUTES_PER_HOUR);
        let hours = self
            .minutes
            .div_euclid(MINUTES_PER_HOUR)
            .rem_euclid(HOURS_PER_DAY);

        f.write_fmt(format_args!("{:02}:{:02}", hours, minutes))
    }
}
