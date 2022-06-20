use cosmwasm_std::Uint128;
use cw_storage_plus::{Map,Item};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::msg::{JunoFarmingMsg};

pub const CONFIG: Item<State> = Item::new("config_state");
pub const METADATA: Item<Vec<JunoFarmingMsg>> = Item::new("metadata");

pub const USERINFO: Map<&str, Uint128> = Map::new("user_info");
pub const WHITEINFO: Map<&str, UserInfo> = Map::new("config_user_info");
pub const ADMININFO: Map<&str, UserInfo> = Map::new("config_admin_info");
pub const TOKENID : Item<Vec<String>>= Item::new("token_id");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_nft:Uint128,
    pub owner:String,
    pub count : Uint128,
    pub check_mint:Vec<bool>,
    pub nft_address:String,
    pub url :String,
    pub image_url:String,
    pub price:Uint128,
    pub denom:String,
    pub max_nft:Uint128
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserInfo {
    pub first_name : Option<String>,
    pub last_name: Option<String>,
    pub email : Option<String>,
    pub mobile : Option<String>,
    pub contract_id:Option<String>,
    pub build_type:Option<String>,
    pub role : Option<String>,
    pub address:String
}

