use std::error;
use std::fmt;
use chrono::{DateTime, Local, TimeZone, Utc};
use regex::Regex;

#[derive(Debug)]
pub struct IncorrectFormatError {}

impl fmt::Display for IncorrectFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect format")
    }
}

impl error::Error for IncorrectFormatError {
    fn description(&self) -> &str {
        "Incorrect format"
    }
}

pub fn convert_utc_to_date(datetime: DateTime<Utc>) -> String {
    return datetime.format("%Y-%m-%d").to_string();
}

pub fn convert_datetime_to_date_and_time(datetime: DateTime<Local>) -> String {
    return datetime.format("%Y/%m/%d %H:%M").to_string();
}

pub fn convert_japanese_era_to_utc(date_str: &str) -> Result<DateTime<Utc>, IncorrectFormatError> {

    // 元号判定、年、月、日ごとに、末尾のスペースを許容するようにパターンマッチングする
    let re: Regex = Regex::new("^(平成|令和)([元0-9]+)年([0-9]+)月([0-9]+)日( |)+$").unwrap();
    let matches = re.captures(date_str).unwrap();

    if matches.len() == 6 {
        let era: &str = matches.get(1).unwrap().as_str();
        let mut year_str: &str = matches.get(2).unwrap().as_str();
        let month_str: &str = matches.get(3).unwrap().as_str();
        let day_str: &str = matches.get(4).unwrap().as_str();

        if year_str == "元" {
            year_str = "1";
        }

        let mut year: i32 = year_str.parse::<i32>().unwrap();
        let month: u32 = month_str.parse::<u32>().unwrap();
        let day: u32 = day_str.parse::<u32>().unwrap();

        match era {
            "平成" => {
                year += 1988;
            },
            "令和" => {
                year += 2018;
            }
            _ => return Err(IncorrectFormatError {})
        }

        return Ok(Utc.ymd(year, month, day).and_hms(8, 0, 0));
    }

    return Err(IncorrectFormatError {})

}
