use core::fmt::Display;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Api, StdResult};
use cw20::Denom;

#[cw_serde]
#[derive(Eq, Ord, PartialOrd)]
pub enum SerializableDenom {
    Native(String),
    Cw20(String),
}

impl Display for SerializableDenom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SerializableDenom::Native(denom) => format!("native_{}", denom).to_string(),
            SerializableDenom::Cw20(contract_address) => {
                format!("cw20_{}", contract_address).to_string()
            }
        };
        write!(f, "{}", str)
    }
}

impl From<Denom> for SerializableDenom {
    fn from(value: Denom) -> Self {
        match value {
            Denom::Native(denom) => SerializableDenom::Native(denom),
            Denom::Cw20(contract_address) => SerializableDenom::Cw20(contract_address.to_string()),
        }
    }
}

impl SerializableDenom {
    pub fn to_denom(&self, api: &dyn Api) -> StdResult<Denom> {
        Ok(match self {
            SerializableDenom::Native(denom) => Denom::Native(denom.clone()),
            SerializableDenom::Cw20(contract_address) => {
                Denom::Cw20(api.addr_validate(contract_address)?)
            }
        })
    }
}
