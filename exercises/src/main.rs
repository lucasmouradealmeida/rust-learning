mod reverse_string;
mod gigasecond;
use time::PrimitiveDateTime as DateTime;


fn main() {
    reverse_string::reverse("hello");
    gigasecond::after(dt(2011, 4, 25, 0, 0, 0));
}


fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    use time::{Date, Time};
    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}

