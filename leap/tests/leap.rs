use leap;

fn process_leapyear_case(year: u64, expected: bool) {
    assert_eq!(leap::is_leap_year(year), expected);
}

#[test]
fn test_year_not_divisible_by_4_common_year() {
    process_leapyear_case(2015, false);
}

#[test]

fn test_year_divisible_by_2_not_divisible_by_4_in_common_year() {
    process_leapyear_case(1970, false);
}

#[test]

fn test_year_divisible_by_4_not_divisible_by_100_leap_year() {
    process_leapyear_case(1996, true);
}

#[test]

fn test_year_divisible_by_4_and_5_is_still_a_leap_year() {
    process_leapyear_case(1960, true);
}

#[test]

fn test_year_divisible_by_100_not_divisible_by_400_common_year() {
    process_leapyear_case(2100, false);
}

#[test]

fn test_year_divisible_by_100_but_not_by_3_is_still_not_a_leap_year() {
    process_leapyear_case(1900, false);
}

#[test]

fn test_year_divisible_by_400_leap_year() {
    process_leapyear_case(2000, true);
}

#[test]

fn test_year_divisible_by_400_but_not_by_125_is_still_a_leap_year() {
    process_leapyear_case(2400, true);
}

#[test]

fn test_year_divisible_by_200_not_divisible_by_400_common_year() {
    process_leapyear_case(1800, false);
}

#[test]

fn test_any_old_year() {
    assert_eq!(leap::is_leap_year(1997), false);
}

#[test]

fn test_early_years() {
    assert_eq!(leap::is_leap_year(1), false);
    assert_eq!(leap::is_leap_year(4), true);
    assert_eq!(leap::is_leap_year(100), false);
    assert_eq!(leap::is_leap_year(400), true);
    assert_eq!(leap::is_leap_year(900), false);
}

#[test]

fn test_century() {
    assert_eq!(leap::is_leap_year(1700), false);
    assert_eq!(leap::is_leap_year(1800), false);
    assert_eq!(leap::is_leap_year(1900), false);
}

#[test]

fn test_exceptional_centuries() {
    assert_eq!(leap::is_leap_year(1600), true);
    assert_eq!(leap::is_leap_year(2000), true);
    assert_eq!(leap::is_leap_year(2400), true);
}

#[test]
fn test_years_1600_to_1699() {
    let incorrect_years = (1600..1700)
        .filter(|&year| leap::is_leap_year(year) != (year % 4 == 0))
        .collect::<Vec<_>>();

    if !incorrect_years.is_empty() {
        panic!("incorrect result for years: {:?}", incorrect_years);
    }
}
