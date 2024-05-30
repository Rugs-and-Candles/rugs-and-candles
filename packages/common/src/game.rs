use cosmwasm_schema::cw_serde;

use crate::errors::ContractError;

#[cw_serde]
pub enum Chains {
    Neutron,
    Kujira,
    Osmosis,
    Stargaze,
    NonSupported,
}

impl From<&String> for Chains {
    fn from(s: &String) -> Self {
        match s.to_lowercase().as_str() {
            "neutron" => Chains::Neutron,
            "harpoon" => Chains::Kujira,
            "kaiyo" => Chains::Kujira,
            "osmosis" => Chains::Osmosis,
            "stargaze" => Chains::Stargaze,
            _ => Chains::NonSupported,
        }
    }
}

impl Chains {
    pub fn is_valid(&self) -> Result<(), ContractError> {
        use Chains::*;
        match self {
            NonSupported => Err(ContractError::Unauthorized {}),
            _ => Ok(()),
        }
    }

    // pub fn validate_tiles(&self, tiles) -> Option<String> {
    //     if let Chains::Neutron = self {
    //         Some("Specific method for Neutron".to_string())
    //     } else {
    //         None
    //     }
    // }
}
