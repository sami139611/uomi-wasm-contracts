
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw_storage_plus::Map;
use serde::{Deserialize, Serialize};

// پیام‌های قرارداد
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    Set { key: String, value: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    Get { key: String },
}

// استوریج دائمی
const STORE: Map<&str, String> = Map::new("store");

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: (),
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Set { key, value } => {
            STORE.save(deps.storage, &key, &value)?;
            Ok(Response::new().add_attribute("method", "set"))
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Get { key } => {
            let result = STORE.may_load(deps.storage, &key)?;
            to_binary(&result)
        }
    }
  }
