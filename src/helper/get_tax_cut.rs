use crate::constant::data_ptkp::DATA_PTKP;

pub fn get_tax_cut(is_already_married: &bool, number_of_children: &u32) -> u128 {
    if !(*is_already_married) {
        return DATA_PTKP.tk;
    }

    match number_of_children {
        0 => DATA_PTKP.k0,
        1 => DATA_PTKP.k1,
        2 => DATA_PTKP.k2,
        _ => DATA_PTKP.k3,
    }
}
