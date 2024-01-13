use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self(minutes + hours * 60)
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        self.0 = self.0 + minutes;
        self.clone()
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut hours = (self.0 / 60).rem_euclid(24);
        let minutes = self.0.rem_euclid(60);

        if self.0.is_negative() && minutes != 0 {
            hours -= 1;
            hours = hours.rem_euclid(24);
        }

        write!(f, "{hours:02}:{minutes:02}")
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.0.rem_euclid(24 * 60)
            == other.0.rem_euclid(24 * 60)
    }
}
