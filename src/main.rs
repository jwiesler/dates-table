use chrono::format::{parse, Item, Numeric, Pad, Parsed};
use chrono::{Datelike, NaiveDate, ParseResult, Weekday};

use std::env::args;

fn parse_month(s: &str) -> ParseResult<NaiveDate> {
    const ITEMS: &[Item<'static>] = &[
        Item::Numeric(Numeric::Year, Pad::Zero),
        Item::Literal("-"),
        Item::Numeric(Numeric::Month, Pad::Zero),
    ];

    let mut parsed = Parsed::new();
    parse(&mut parsed, s, ITEMS.iter())?;
    parsed.set_day(1)?;
    parsed.to_naive_date()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_weekday(day: &NaiveDate) -> bool {
    !matches!(day.weekday(), Weekday::Sat | Weekday::Sun)
}

fn weekday_to_str(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

fn main() {
    let arg = args().nth(1).expect("Expected one arg");
    let date = parse_month(&arg).expect("parse month, expected format YYYY-MM");
    let month = date.iter_days().take_while(|d| d.month() == date.month());
    for date in month.filter(is_weekday) {
        let year = date.year();
        let month = date.month();
        let day = date.day();
        let week_day = weekday_to_str(date.weekday());
        println!("| {year}-{month:0>2}-{day:0>2} | {week_day: <11} |          |");
    }
}
