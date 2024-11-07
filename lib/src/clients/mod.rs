pub mod serviceinfo;
pub mod tes;
pub mod trs;

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
