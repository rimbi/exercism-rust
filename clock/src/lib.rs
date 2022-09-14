use std::fmt::Display;


const HOUR: i32 = 60;
const DAY: i32 = HOUR * 24;

#[derive(PartialEq, Debug)]
pub struct Clock(i32, i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = ((hours * HOUR + minutes) % DAY + DAY) % (DAY);
        Self(minutes / HOUR, minutes % HOUR)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.0, self.1 + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.0, self.1)
    }
}
