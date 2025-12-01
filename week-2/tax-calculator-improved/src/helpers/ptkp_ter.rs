use crate::constants::ptkp::Ptkp;
use crate::constants::tax::{TaxBracket,TER_A_BRACKET,TER_B_BRACKET,TER_C_BRACKET};

pub fn get(ptkp: Ptkp) -> &'static [TaxBracket] {
    match ptkp {
        Ptkp::TK0 | Ptkp::TK1 | Ptkp::K0 => TER_A_BRACKET,
        Ptkp::TK2 | Ptkp::TK3 | Ptkp::K2 | Ptkp::K1 => TER_B_BRACKET,
        Ptkp::K3 => TER_C_BRACKET
    }
}


