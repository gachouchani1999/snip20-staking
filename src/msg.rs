use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Uint128, Binary, HumanAddr, Coin};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub snip20_hash: String,
    pub snip20_address: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    // Receive SNIP20 tokens
    Receive {
        sender: String,
        from: String,
        amount: Uint128,
        #[serde(default)]
        msg: Option<Binary>,
    },  
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}


/// info needed to perform a callback message after instantiation
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct PostInitCallback {
    /// the callback message to execute
    pub msg: Binary,
    /// address of the contract to execute
    pub contract_address: HumanAddr,
    /// code hash of the contract to execute
    pub code_hash: String,
    /// list of native Coin to send with the callback message
    pub send: Vec<Coin>,
}