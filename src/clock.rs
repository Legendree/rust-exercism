use std::ops::Add;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut set_hours: i32;
        let set_minutes: i32;

        if hours >= 24 {
            set_hours = hours % 24;
        } else if hours < 0 {
            let abs_hour = hours.abs();

            let correct_hour = if abs_hour >= 24 {
                24 - abs_hour % 24
            } else {
                (24 + hours).abs()
            };

            set_hours = correct_hour;
        } else {
            set_hours = hours;
        }

        if minutes >= 60 {
            set_minutes = minutes % 60;
            set_hours += (minutes - set_minutes) / 60;

            if set_hours >= 24 {
                set_hours = set_hours % 24;
            }
        } else if minutes < 0 {
            let abs_min = minutes.abs();

            let minutes_left = abs_min % 60;

            let to_subtruct_hours = if minutes_left > 0 {
                (abs_min - minutes_left) / 60 + 1
            } else {
                abs_min / 60
            };

            let new_hour = set_hours - to_subtruct_hours;

            if new_hour < 0 || new_hour >= 24 {
                set_hours = 24 - new_hour.abs() % 24
            } else {
                set_hours -= to_subtruct_hours;
            }

            let real_minutes = 60 - abs_min % 60;

            set_minutes = if real_minutes == 60 { 0 } else { real_minutes };
        } else {
            set_minutes = minutes;
        }

        Clock {
            hours: set_hours,
            minutes: set_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        let mut set_hours = self.hours;

        let set_minutes: i32;

        if minutes >= 60 {
            set_minutes = minutes % 60;
            set_hours += (minutes - set_minutes) / 60;

            if set_hours >= 24 {
                set_hours = set_hours % 24;
            }
        } else if minutes < 0 {
            let abs_min = minutes.abs();

            let minutes_left = abs_min % 60;

            let to_subtruct_hours = if minutes_left > 0 {
                (abs_min - minutes_left) / 60 + 1
            } else {
                abs_min / 60
            };

            set_hours = if self.hours - to_subtruct_hours < 0 {
                24 - (self.hours - to_subtruct_hours).abs() % 24
            } else {
                self.hours - to_subtruct_hours
            };

            let real_minutes = 60 - abs_min % 60;

            set_minutes = if real_minutes == 60 { 0 } else { real_minutes };
        } else {
            set_minutes = minutes;
        }

        return Clock {
            hours: set_hours,
            minutes: set_minutes,
        };
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = if self.hours <= 9 {
            format!("0{}", self.hours)
        } else {
            format!("{}", self.hours)
        };

        let minutes = if self.minutes <= 9 {
            format!("0{}", self.minutes)
        } else {
            format!("{}", self.minutes)
        };

        write!(f, "{}:{}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
