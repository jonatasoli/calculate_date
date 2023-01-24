use chrono::{Duration, NaiveDate};

pub fn calculate_days_between_dates(
    start_period: Option<NaiveDate>,
    finish_period: Option<NaiveDate>,
) -> i64 {
    let start_date = start_period.expect("Start Period is not valid date!");
    let end_date = finish_period.expect("End Period is not valid date!");
    let duration = end_date - start_date;
    duration.num_days()
}

pub fn calculate_date_with_date_plus_days(start_period: Option<NaiveDate>, days: i64) -> NaiveDate {
    let start_date = start_period.expect("Start Period is not valid date!");
    start_date + Duration::days(days)
}

pub fn calculate_date_with_date_minus_days(
    start_period: Option<NaiveDate>,
    days: i64,
) -> NaiveDate {
    let start_date = start_period.expect("Start Period is not valid date!");
    start_date - Duration::days(days)
}
