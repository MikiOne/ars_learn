use chrono::{Local, NaiveDateTime};
use chrono::format::strftime::StrftimeItems;

pub fn now_to_ymd() -> String {
    let now = Local::now().naive_local();
    let ymd_fmt = StrftimeItems::new("%Y%m%d");
    now.format_with_items(ymd_fmt.clone()).to_string()
}

pub fn to_ymd(date_time: NaiveDateTime) -> String {
    let ymd_fmt = StrftimeItems::new("%Y-%m-%d");
    date_time.format_with_items(ymd_fmt.clone()).to_string()
}

pub fn now() -> String {
    // Local::now().format("%F %T").to_string()
    Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}


#[test]
fn test_now_to_ymd() {
    println!("now ymd: {}", now_to_ymd());
}


#[test]
fn test_to_ymd() {
    // let date_time = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);
    // println!("result: {}", to_ymd(date_time));

    // let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
    // assert_eq!(dt.format_with_items(fmt.clone()).to_string(), "2015-09-05 23:56:04");
    // assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(),    "2015-09-05 23:56:04");
    //
    // assert_eq!(format!("{}", dt.format_with_items(fmt)), "2015-09-05 23:56:04");
}