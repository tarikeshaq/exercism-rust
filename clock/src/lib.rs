use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: u32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = (hours * 60 + minutes) % 1_440;
        if minutes < 0 {
            minutes += 1_440;
        }
        Clock {
            minutes: minutes as u32,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, minutes + (self.minutes as i32))
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let hours = (self.minutes / 60) % 24;
        let minutes = (self.minutes - (hours * 60)) % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
