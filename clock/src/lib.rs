use std::fmt;

#[derive(fmt::Debug)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut m = minutes % 60;
        if minutes.is_negative() {
            m = (minutes % 60) + 60;
        }
        let carry = minutes / 60;
        let mut h = (hours + carry) % 24;
        if hours.is_negative() {
            h = (hours % 24) + 24 + carry;
        }
        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = (self.minutes + minutes) % 60;
        let carry = (self.minutes + minutes) / 60;
        let h = (self.hours + carry) % 24;
        Clock {
            hours: h,
            minutes: m,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
impl Eq for Clock {}
