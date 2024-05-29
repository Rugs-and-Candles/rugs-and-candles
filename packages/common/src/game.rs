use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum SupportedChains {
    Neutron,
    Kujira,
    Osmosis,
    Stargaze,
}

impl From<String> for SupportedChains {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "neutron" => SupportedChains::Neutron,
            "kujira" => SupportedChains::Kujira,
            "osmosis" => SupportedChains::Osmosis,
            "stargaze" => SupportedChains::Stargaze,
            _ => panic!("Unsupported chain: {}", s),
        }
    }
}

impl SupportedChains {
    // Method common to all variants
    pub fn common_method(&self) -> String {
        match self {
            SupportedChains::Neutron => "Common method for Neutron".to_string(),
            SupportedChains::Kujira => "Common method for Kujira".to_string(),
            SupportedChains::Osmosis => "Common method for Osmosis".to_string(),
            SupportedChains::Stargaze => "Common method for Stargaze".to_string(),
        }
    }

    // Method specific to Neutron
    pub fn neutron_specific_method(&self) -> Option<String> {
        if let SupportedChains::Neutron = self {
            Some("Specific method for Neutron".to_string())
        } else {
            None
        }
    }
}
