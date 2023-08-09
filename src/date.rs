use time;
use time::Duration;

// construct deadline date into string
pub fn construct_date(offset: u8) -> String {
    let deadline = add_days_to_current_date(offset);
    let result: String = format!("{}-{}", deadline.day(), deadline.month());
    result
}

// calculate deadline date
fn add_days_to_current_date(offset: u8) -> time::OffsetDateTime {
    let deadline = time::OffsetDateTime::now_utc().checked_add(Duration::days(offset as i64));
    if let Some(date) = deadline {
        return date;
    }
    // return today date if overflowed
    time::OffsetDateTime::now_utc()
}

#[cfg(test)]
mod tests {
    use crate::date::construct_date;
    use time::macros::datetime;

    #[test]
    fn today_date() {
        println!("{}", time::OffsetDateTime::now_utc());
        assert!(time::OffsetDateTime::now_utc().year() >= 2022);
    }

    #[test]
    fn date_offset() {
        assert!(crate::date::add_days_to_current_date(5) > time::OffsetDateTime::now_utc());
    }

    #[test]
    fn check_string_formatting() {
        assert_eq!(construct_date(0), "5-August");
    }
}
