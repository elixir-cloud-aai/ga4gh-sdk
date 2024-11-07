pub mod serviceinfo;
pub mod tes;

#[derive(Debug)]
pub enum ServiceType {
    TES,
    DRS,
    TRS,
    AAI,
}

impl ServiceType {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceType::TES => "TES",
            ServiceType::DRS => "DRS",
            ServiceType::TRS => "TRS",
            ServiceType::AAI => "AAI",
        }
    }
}

use std::fmt;

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}