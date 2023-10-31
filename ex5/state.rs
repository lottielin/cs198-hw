use serde::{Deserialize, Serialize};
use universe::species::{SapienceScale, Sapient};
use cosmwasm_std::{Addr};

// add the derive attribute here
#[derive(Serialize, Deserialize)]
// add the State struct here (the last member will end with a comma)
pub struct State {
    pub owner: Addr,
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}