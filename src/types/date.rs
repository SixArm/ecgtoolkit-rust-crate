// Date type
//
// Date class for ECG records (format matches SCP).
// Original author: Maarten JB van Ettinger.

/// Date with year, month, day (format matches SCP).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    /// Days in each month (index 0 unused).
    const DAYS_IN_MONTH: [u8; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const LEAP_MONTH: u8 = 2;
    const DAYS_IN_LEAP_MONTH: u8 = 29;

    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    /// Check if date is likely to be an existing date.
    pub fn is_existing_date(&self) -> bool {
        if self.month > 0 && self.month <= 12 && self.year > 0 {
            if self.month == Self::LEAP_MONTH
                && (self.year % 4) == 0
                && ((self.year % 100) != 0 || (self.year % 400) == 0)
            {
                self.day > 0 && self.day <= Self::DAYS_IN_LEAP_MONTH
            } else {
                self.day > 0 && self.day <= Self::DAYS_IN_MONTH[self.month as usize]
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_existing_date() {
        assert!(Date::new(2024, 1, 15).is_existing_date());
        assert!(Date::new(2024, 2, 29).is_existing_date()); // leap year
        assert!(!Date::new(2023, 2, 29).is_existing_date()); // not leap year
        assert!(!Date::new(0, 1, 1).is_existing_date());
        assert!(!Date::new(2024, 13, 1).is_existing_date());
        assert!(!Date::new(2024, 1, 32).is_existing_date());
    }
}
