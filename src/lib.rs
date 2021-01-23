mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compound_interest(principle: f32, annual_rate: f32, years: i32, times_compounded_per_year: i32) -> f32 {
    principle * (1.0 + (annual_rate / times_compounded_per_year as f32)).powi(times_compounded_per_year * years)
}

#[wasm_bindgen]
pub fn compound_interest_formatted(principle: f32, annual_rate: f32, years: i32, times_compounded_per_year: i32) -> String {
    let value = compound_interest(principle, annual_rate, years, times_compounded_per_year);
    format!("${:.2}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_compound_interest_example() {
        let principle = 1500.0;
        let rate = 0.043;
        let years = 6;
        let times_compounded_per_year = 4;

        let rounded_result = (compound_interest(principle, rate, years, times_compounded_per_year) * 100.0).round() / 100.0;
        assert_eq!(1938.84, rounded_result);
    }

    #[test]
    pub fn test_compound_interest_formatted() {
        let principle = 1500.0;
        let rate = 0.043;
        let years = 6;
        let times_compounded_per_year = 4;

        let result = compound_interest_formatted(principle, rate, years, times_compounded_per_year);
        assert_eq!("$1938.84", result);
    }
}
