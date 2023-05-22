use std::fmt::Formatter;

const HOUR: i32 = 60;
const DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: ((hours * HOUR + minutes) % DAY + DAY) % DAY
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(
            0,
            self.minutes + minutes,
        )
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.minutes / HOUR, self.minutes % HOUR)
    }
}