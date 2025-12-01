use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Ptkp {
    TK0,
    TK1,
    K0,
    TK2,
    TK3,
    K1,
    K2,
    K3
}
	
// Map PTKP string to PTKP enum
impl FromStr for Ptkp {
    type Err = ();
	
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tk0" => Ok(Ptkp::TK0),
            "tk1" => Ok(Ptkp::TK1),
            "k0" => Ok(Ptkp::K0),
            "tk2" => Ok(Ptkp::TK2),
            "tk3" => Ok(Ptkp::TK3),
            "k1" => Ok(Ptkp::K1),
            "k2" => Ok(Ptkp::K2),
            "k3" => Ok(Ptkp::K3),
            _ => Ok(Ptkp::TK0), 
        }
    }
}
