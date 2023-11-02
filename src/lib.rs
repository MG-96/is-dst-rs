use time::{OffsetDateTime, UtcOffset};

/// Return if at a distinct moment in time DST has to be applied to german time.
///
/// DST starts the last sunday of March at 2:00 (UTC +1).
///
/// DST ends the last sunday of October at 3:00 (UTC +2).
///
/// A offset of the input time is ignored.
/// Meaning, if its dst in germany at a given utc time, is independent from the timezone.
pub fn is_german_dst(utc_time: &OffsetDateTime) -> bool {
    // Summertime ist from
    // last Sunday, March, 2:00 to
    // last Sunday, October, 3:00
    //
    // From winter --> summer time: 2:00 --> 3:00
    // From summer --> winter time: 3:00 --> 2:00

    let winter_time = utc_time.to_offset(UtcOffset::from_hms(1, 0, 0).unwrap());

    let month = utc_time.month();
    let day = utc_time.day();
    let weekday = utc_time.weekday();

    match month {
        time::Month::January => false,
        time::Month::February => false,
        time::Month::March => {
            match weekday {
                time::Weekday::Monday => {
                    if day < 26 {
                        false
                    } else {
                        true
                    }
                }
                time::Weekday::Tuesday => {
                    if day < 27 {
                        false
                    } else {
                        true
                    }
                }
                time::Weekday::Wednesday => {
                    if day < 28 {
                        false
                    } else {
                        true
                    }
                }
                time::Weekday::Thursday => {
                    if day < 29 {
                        false
                    } else {
                        true
                    }
                }
                time::Weekday::Friday => {
                    if day < 30 {
                        false
                    } else {
                        true
                    }
                }
                time::Weekday::Saturday => {
                    if day < 31 {
                        false
                    } else {
                        true
                    }
                }
                time::Weekday::Sunday => {
                    if day < 25 {
                        // not the last sunday
                        false
                    } else {
                        // is the last sunday
                        if winter_time.hour() < 2 {
                            false
                        } else {
                            true
                        }
                    }
                }
            }
        }
        time::Month::April => true,
        time::Month::May => true,
        time::Month::June => true,
        time::Month::July => true,
        time::Month::August => true,
        time::Month::September => true,
        time::Month::October => {
            match weekday {
                time::Weekday::Monday => {
                    if day < 26 {
                        true
                    } else {
                        false
                    }
                }
                time::Weekday::Tuesday => {
                    if day < 27 {
                        true
                    } else {
                        false
                    }
                }
                time::Weekday::Wednesday => {
                    if day < 28 {
                        true
                    } else {
                        false
                    }
                }
                time::Weekday::Thursday => {
                    if day < 29 {
                        true
                    } else {
                        false
                    }
                }
                time::Weekday::Friday => {
                    if day < 30 {
                        true
                    } else {
                        false
                    }
                }
                time::Weekday::Saturday => {
                    if day < 31 {
                        true
                    } else {
                        false
                    }
                }
                time::Weekday::Sunday => {
                    if day < 25 {
                        // not the last sunday
                        true
                    } else {
                        // is the last sunday
                        if winter_time.hour() < 2 {
                            true
                        } else {
                            false
                        }
                    }
                }
            }
        }
        time::Month::November => false,
        time::Month::December => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_is_dst_winter_to_summer() {
        let wintertime = datetime!(2023-03-26 1:59 +1);
        let summertime = datetime!(2023-03-26 2:00 +1);
        assert_eq!(is_german_dst(&wintertime), false);
        assert_eq!(is_german_dst(&summertime), true);
    }

    #[test]
    fn test_is_dst_summer_to_winter() {
        let summertime = datetime!(2023-10-29 2:59 +2);
        let wintertime = datetime!(2023-10-29 3:00 +2);
        assert_eq!(is_german_dst(&wintertime), false);
        assert_eq!(is_german_dst(&summertime), true);

        let first_minute_in_winter = datetime!(2023-10-29 2:00 +1);
        assert_eq!(is_german_dst(&first_minute_in_winter), false);
    }

    #[test]
    fn test_various_weekdays() {
        let summertime = datetime!(2023-10-22 3:00 +2); // last sunday with summertime
        let summertime2 = datetime!(2023-10-28 3:00 +2); // last saturday with summertime
        let wintertime = datetime!(2023-10-29 3:00 +2);
        assert_eq!(is_german_dst(&wintertime), false);
        assert_eq!(is_german_dst(&summertime), true);
        assert_eq!(is_german_dst(&summertime2), true);
    }
}
