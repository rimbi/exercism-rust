use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: Self::normalize(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            minutes: Self::normalize(self.minutes + minutes),
        }
    }

    fn normalize(mut minutes: i32) -> i32 {
        const ONE_DAY: i32 = 24 * 60;
        while minutes < 0 {
            minutes += ONE_DAY;
        }
        while minutes >= ONE_DAY {
            minutes -= ONE_DAY;
        }
        minutes
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minutes = self.minutes;
        write!(f, "{:0>2}:{:0>2}", minutes / 60, minutes % 60)
    }
}
