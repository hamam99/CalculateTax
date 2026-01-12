use crate::helper::get_tax_cut::get_tax_cut;
use crate::helper::get_tax_rate::get_tax_rate;

pub fn get_salary_taxed(
    salary_per_month: &u32,
    is_already_married: &bool,
    number_of_children: &u32,
) -> [u128; 2] {
    let salary_per_year: u128 = (salary_per_month * 12) as u128;
    let total_tax_cut: u128 = get_tax_cut(is_already_married, number_of_children);
    let salary_taxed: u128 = salary_per_year.saturating_sub(total_tax_cut);

    if salary_taxed == 0 {
        return [0, 0];
    }

    let tax_rate = get_tax_rate(&salary_taxed);
    let tax_to_pay: u128 = salary_taxed * tax_rate as u128 / 100;
    [salary_taxed, tax_to_pay]
}
