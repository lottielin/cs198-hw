use serde::{Deserialize, Serialize};
use universe::species::{SapienceScale, Sapient};

#[derive(Serialize, Deserialize)]
// add the struct here
pub struct InstantiateMsg {
    pub planet_name: String,
}