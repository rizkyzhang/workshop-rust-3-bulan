use crate::constants::ptkp::Ptkp;

pub fn get(ptkp: Ptkp) -> u32 {
    match ptkp {
        Ptkp::TK0 => 54_000_000,
        Ptkp::TK1 | Ptkp::K0 => 58_500_000,
        Ptkp::TK2 | Ptkp::K1 => 63_000_000,
        Ptkp::TK3 | Ptkp::K2 => 67_500_000,
        Ptkp::K3 => 72_000_000
    }
}
