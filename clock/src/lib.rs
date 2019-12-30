use std::fmt::{Display, Formatter, Result};

fn fix_time(time: i32, max_count: i32) -> u8 {
    let time = time % max_count;
    if time >= 0 {
        time as u8
    } else {
        ((max_count + time) % max_count) as u8
    }
}

fn normalize_time(hours: i32, minutes: i32) -> (u8, u8) {
    let hours_diff = minutes / 60;
    let mut hours = hours + hours_diff;
    let minutes = minutes % 60;
    if minutes < 0 {
        hours -= 1;
    }
    (fix_time(hours, 24), fix_time(minutes, 60))
}

#[derive(Debug)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = normalize_time(hours, minutes);
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours as i32, minutes + self.minutes as i32)
    }
}

fn format_time(time: u8) -> String {
    if time > 9 {
        time.to_string()
    } else {
        let mut res = String::from("0");
        res.push_str(&time.to_string());
        res
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}:{}",
            format_time(self.hours),
            format_time(self.minutes)
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
