use crate::constants::tax::TaxBracket;

pub fn get(
    gross_income: u64, 
    ter: &'static [TaxBracket]
) -> Option<f64> {
    ter
        .iter()
        .find(|ter| gross_income < ter.income_up_to)
        .map(|ter| ter.percentage)
}
