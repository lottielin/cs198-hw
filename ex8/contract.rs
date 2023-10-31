use cosmwasm_std::{
    entry_point, DepsMut, Env, MessageInfo, Response
};
use crate::error::ContractError;
use crate::msg::{InstantiateMsg};
use crate::state::{config, State};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // initialize the state variable here
    let new_state = State {
        owner: info.sender,
        planet_name: msg.planet_name,
        planet_sapients: vec![],
        minimum_sapience: SapienceScale::Low,
    };
    // use config to save the reference to state
    config(deps.storage).save(&new_state)?;
    Ok(Response::default())
}