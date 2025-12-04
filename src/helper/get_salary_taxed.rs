#[path = "../helper/get_tax_rate.rs"]
mod get_tax_rate;
use self::get_tax_rate::get_tax_rate;

#[path = "../helper/get_tax_cut.rs"]
mod get_tax_cut;
use self::get_tax_cut::get_tax_cut;

pub fn get_salary_taxed(salary_per_month: &u32, is_already_married: &bool, number_of_children: &u32) -> [u128; 2] {

  let salary_per_year:u128 = (salary_per_month * 12) as u128;
  let total_tax_cut:u128 = get_tax_cut(&is_already_married, &number_of_children) as u128;
  let salary_taxed:u128 = salary_per_year.saturating_sub(total_tax_cut);
  
  if salary_taxed <= 0 {
    return [0, 0];
  }

  let tax_rate = get_tax_rate(&salary_taxed);
  let tax_to_pay:u128 = salary_taxed * tax_rate as u128 / 100;
  return  [salary_taxed, tax_to_pay]
}