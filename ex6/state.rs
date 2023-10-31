use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{ReadonlySingleton, Singleton};
use universe::species::{SapienceScale, Sapient};

// add the config signature here
pub fn config(storage: &mut dyn Storage) -> Singleton<State> {}

// add the config_read signature here
pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub owner: Addr,
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}