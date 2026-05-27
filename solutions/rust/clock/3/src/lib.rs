use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    pub hours: i32,
    pub minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Self {
        //     hours: if hours < 0 {24 + (hours % 24) } else { hours % 24 },
        //     minutes: if minutes < 0 { 60 + (minutes % 60) } else { minutes % 60 },
        // }

        let total_minutes = hours * 60 + minutes;

        let minutes_in_day = 24 * 60;

        let normalized = total_minutes.rem_euclid(minutes_in_day);

        let hours = normalized / 60;
        let minutes = normalized % 60;

        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // let mut h = self.hours;
        // let mut m = self.minutes;
        // for i in 1..minutes+1 {
        //     m = m + 1;
        //     if m == 60 {
        //         h += 1;
        //         if h == 24 {
        //             h = 0;
        //         }
        //         m = 0
        //     }
        // }
        // Self {
        //     hours: h,
        //     minutes: m
        // }
        // let mut tol_min = self.minutes + minutes;
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
