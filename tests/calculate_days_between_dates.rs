use calculate_date::calculate_days_between_dates;
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
