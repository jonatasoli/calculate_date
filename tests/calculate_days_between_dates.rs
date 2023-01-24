use calculate_date::{
    calculate_date_with_date_minus_days, calculate_date_with_date_plus_days,
    calculate_days_between_dates,
};
use chrono::NaiveDate;

#[test]
fn test_given_valid_date_and_days_should_return_days() {
    let start_period = NaiveDate::from_ymd_opt(2020, 1, 1);
    let end_period = NaiveDate::from_ymd_opt(2020, 1, 2);
    assert_eq!(calculate_days_between_dates(start_period, end_period), 1);
}

#[test]
fn test_given_invalid_start_period_should_return_invalid_date_error() {
    let start_period = NaiveDate::from_ymd_opt(0, 0, 0);
    let end_period = NaiveDate::from_ymd_opt(2020, 1, 2);
    let result =
        std::panic::catch_unwind(|| calculate_days_between_dates(start_period, end_period));
    assert_eq!(result.is_err(), true);
}

#[test]
fn test_given_invalid_end_period_should_return_invalid_date_error() {
    let start_period = NaiveDate::from_ymd_opt(2020, 1, 2);
    let end_period = NaiveDate::from_ymd_opt(0, 0, 0);
    let result =
        std::panic::catch_unwind(|| calculate_days_between_dates(start_period, end_period));
    assert_eq!(result.is_err(), true);
}

#[test]
#[should_panic(expected = "Start Period is not valid date!")]
fn test_given_invalid_period_should_return_panic_message() {
    let start_period = NaiveDate::from_ymd_opt(0, 0, 0);
    let end_period = NaiveDate::from_ymd_opt(2020, 1, 2);
    calculate_days_between_dates(start_period, end_period);
}

#[test]
fn test_given_start_date_and_10_days_should_return_new_date_with_10_days_in_front() {
    let start_period = NaiveDate::from_ymd_opt(2020, 1, 1);
    let days = 10;
    let result = calculate_date_with_date_plus_days(start_period, days);
    assert_eq!(
        result,
        NaiveDate::from_ymd_opt(2020, 1, 11).expect("Date not found!")
    )
}

#[test]
fn test_given_start_date_and_negative_days_should_return_new_date_with_sum_negative() {
    let start_period = NaiveDate::from_ymd_opt(2020, 1, 11);
    let days = -10;
    let result = calculate_date_with_date_plus_days(start_period, days);
    assert_eq!(
        result,
        NaiveDate::from_ymd_opt(2020, 1, 1).expect("Date not found!")
    )
}

#[test]
fn test_given_invalid_start_period_in_plus_days_should_return_invalid_date_error() {
    let start_period = NaiveDate::from_ymd_opt(0, 0, 0);
    let result = std::panic::catch_unwind(|| calculate_date_with_date_plus_days(start_period, 10));
    assert_eq!(result.is_err(), true);
}

#[test]
#[should_panic(expected = "Start Period is not valid date!")]
fn test_given_invalid_period_in_calculate_plus_should_return_panic_message() {
    let start_period = NaiveDate::from_ymd_opt(0, 0, 0);
    calculate_date_with_date_plus_days(start_period, 10);
}

#[test]
fn test_given_in_minus_days_start_date_and_10_days_should_return_new_date_with_10_days_in_back() {
    let start_period = NaiveDate::from_ymd_opt(2020, 1, 11);
    let days = 10;
    let result = calculate_date_with_date_minus_days(start_period, days);
    assert_eq!(
        result,
        NaiveDate::from_ymd_opt(2020, 1, 1).expect("Date not found!")
    )
}

#[test]
fn test_given_start_date_and_negative_days_should_return_new_date_with_minus_negative() {
    let start_period = NaiveDate::from_ymd_opt(2020, 1, 1);
    let days = -10;
    let result = calculate_date_with_date_minus_days(start_period, days);
    assert_eq!(
        result,
        NaiveDate::from_ymd_opt(2020, 1, 11).expect("Date not found!")
    )
}

#[test]
fn test_given_invalid_start_period_in_minus_days_should_return_invalid_date_error() {
    let start_period = NaiveDate::from_ymd_opt(0, 0, 0);
    let result = std::panic::catch_unwind(|| calculate_date_with_date_minus_days(start_period, 10));
    assert_eq!(result.is_err(), true);
}

#[test]
#[should_panic(expected = "Start Period is not valid date!")]
fn test_given_invalid_period_in_calculate_minus_should_return_panic_message() {
    let start_period = NaiveDate::from_ymd_opt(0, 0, 0);
    calculate_date_with_date_minus_days(start_period, 10);
}
