use compound_interest_calculator;

#[test]
fn it_calculates_compound_interest() {
    let principle = 1500.0;
    let rate = 0.043;
    let years = 6;
    let times_compounded_per_year = 4;

    let rounded_result = (compound_interest_calculator::compound_interest(principle, rate, years, times_compounded_per_year) * 100.0).round() / 100.0;
    assert_eq!(1938.84, rounded_result);
}

