use cosmwasm_schema::cw_serde;
use std::convert::TryFrom;

use crate::errors::BoardError;

#[cw_serde]
pub enum Chains {
    Neutron,
    Kujira,
    Osmosis,
    Stargaze,
    NonSupported,
}

impl TryFrom<&str> for Chains {
    type Error = BoardError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_str() {
            "neutron" => Ok(Chains::Neutron),
            "harpoon" => Ok(Chains::Kujira),
            "kaiyo" => Ok(Chains::Kujira),
            "osmosis" => Ok(Chains::Osmosis),
            "stargaze" => Ok(Chains::Stargaze),
            _ => Err(BoardError::InvalidChain(s.into())),
        }
    }
}

impl Chains {
    // pub fn validate_tiles(&self, tiles) -> Option<String> {
    //     if let Chains::Neutron = self {
    //         Some("Specific method for Neutron".to_string())
    //     } else {
    //         None
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_from_works() {
        let valid_chain = "neutron";
        let res = Chains::try_from(valid_chain);
        assert!(res.is_ok());

        let invalid_chain = "celestia";
        let res = Chains::try_from(invalid_chain);
        assert!(res.is_err());
    }
}
