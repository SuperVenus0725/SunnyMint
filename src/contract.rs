use cosmwasm_std::{
    entry_point, to_binary,   CosmosMsg, Deps, DepsMut,Binary,
    Env, MessageInfo, Response, StdResult, Uint128, WasmMsg,BankMsg,Coin,Order
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg,JunoFarmingMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    CONFIG,State,METADATA, USERINFO, UserInfo, WHITEINFO, TOKENID, ADMININFO
};

use cw721_base::{ExecuteMsg as Cw721BaseExecuteMsg, MintMsg};


#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
         total_nft:msg.total_nft,
         owner:msg.owner,
         count:Uint128::new(0),
         check_mint:msg.check_mint,
         nft_address:"nft".to_string(),
         url : msg.url,
         price:msg.price,
         image_url:msg.image_url,
         denom:msg.denom,
         max_nft:msg.max_nft
    };
    CONFIG.save(deps.storage, &state)?;
    let metadata:Vec<JunoFarmingMsg> = vec![];
    METADATA.save(deps.storage,&metadata)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint{rand} => execute_mint(deps, env, info,rand),
        ExecuteMsg::SetNftAddress { address } => execute_set_nft_address(deps, info, address),
        ExecuteMsg::ChangeOwner { address } => execute_chage_owner(deps, info, address),
        ExecuteMsg::ChangePrice { amount }=> execute_change_price(deps,info,amount),
        ExecuteMsg::AddAdmin { admin } => execute_add_admin(deps,info,admin),
        ExecuteMsg::DeleteAdmin { admin } => execute_delete_admin(deps,info,admin),
        ExecuteMsg::AddWhiteUser { user } =>  execute_add_user(deps,info,user),
        ExecuteMsg::DeleteWhiteUser { user } =>  execute_delete_user(deps,info,user),
    }
}

fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    rand:Uint128
) -> Result<Response, ContractError> {
    let  state = CONFIG.load(deps.storage)?;


    if state.count >= state.total_nft {
        return Err(ContractError::MintEnded {});
    }

    if rand > state.total_nft{
        return Err(ContractError::WrongNumber {  });
    }

    if info.sender.to_string() ==state.owner  {
        let sender = info.sender.to_string();
        let token_id = ["LUNAR_NFT".to_string(),rand.to_string()].join(".");
        
        let mut state = CONFIG.load(deps.storage)?;
        state.count = state.count+Uint128::new(1);
        state.check_mint[Uint128::u128(&rand) as usize -1] = false;
        CONFIG.save(deps.storage, &state)?;

        let ids = TOKENID.may_load(deps.storage)?;
        if ids ==None{
            TOKENID.save(deps.storage, &vec![token_id.clone()])?;
        }

        else {
            TOKENID.update(deps.storage,
            |mut ids|-> StdResult<_>{
                ids.push(token_id.clone());
                Ok(ids)
            }
        )?;
        }

        Ok(Response::new()
            .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: state.nft_address,
                msg: to_binary(&Cw721BaseExecuteMsg::Mint(MintMsg {
                    //::<Metadata>
                    token_id: token_id.clone(),
                    owner: sender,
                    token_uri: Some([[state.url,rand.to_string()].join(""),"json".to_string()].join(".")),
                    extension:  JunoFarmingMsg{
                        image:Some([[state.image_url,rand.to_string()].join(""),"png".to_string()].join("."))
                    }
                }))?,
                funds: vec![],
            }))
        )
    }

    else{

        let white_user = WHITEINFO.may_load(deps.storage, &info.sender.to_string())?;
    
        if white_user == None{
            return Err(ContractError::WrongWhiteUser {  });
        }


        let amount= info
            .funds
            .iter()
            .find(|c| c.denom == state.denom)
            .map(|c| Uint128::from(c.amount))
            .unwrap_or_else(Uint128::zero);

        if amount != state.price{
            return Err(ContractError::Notenough {});
        }

        let sender = info.sender.to_string();
        let token_id = ["LUNAR_NFT".to_string(),rand.to_string()].join(".");
        
        let mut state = CONFIG.load(deps.storage)?;
        state.count = state.count+Uint128::new(1);
        state.check_mint[Uint128::u128(&rand) as usize -1] = false;
        CONFIG.save(deps.storage, &state)?;


        let ids = TOKENID.may_load(deps.storage)?;
        if ids ==None{
            TOKENID.save(deps.storage, &vec![token_id.clone()])?;
        }

        else {
            TOKENID.update(deps.storage,
            |mut ids|-> StdResult<_>{
                ids.push(token_id.clone());
                Ok(ids)
            }
        )?;
        }

        Ok(Response::new()
            .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: state.nft_address,
                msg: to_binary(&Cw721BaseExecuteMsg::Mint(MintMsg {
                    //::<Metadata>
                    token_id: token_id.clone(),
                    owner: sender,
                    token_uri: Some([[state.url,rand.to_string()].join(""),"json".to_string()].join(".")),
                    extension:  JunoFarmingMsg{
                        image:Some([[state.image_url,rand.to_string()].join(""),"png".to_string()].join("."))
                    }
                }))?,
                funds: vec![],
            }))
            .add_message(CosmosMsg::Bank(BankMsg::Send {
                    to_address: state.owner,
                    amount:vec![Coin{
                        denom:state.denom.clone(),
                        amount:amount
                    }]
            }))
        )
    }    
}


fn execute_add_admin(
    deps: DepsMut,
    info: MessageInfo,
    admin: UserInfo,
) -> Result<Response, ContractError> {
    let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string()  {
        return Err(ContractError::Unauthorized {});
        
    }
    ADMININFO.save(deps.storage, &admin.address, &admin)?;
    Ok(Response::default())
}

fn execute_delete_admin(
    deps: DepsMut,
    info: MessageInfo,
    admin: String,
) -> Result<Response, ContractError> {
    let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }

    ADMININFO.remove(deps.storage, &admin);
    
    Ok(Response::default())
}


fn execute_add_user(
    deps: DepsMut,
    info: MessageInfo,
    user: UserInfo,
) -> Result<Response, ContractError> {
    let state =CONFIG.load(deps.storage)?;
    let admin = query_admin_list(deps.as_ref())?;
    if state.owner != info.sender.to_string() {
     match admin.binary_search(&info.sender.to_string())  {
        Ok(u) => {
             WHITEINFO.save(deps.storage, &user.address, &user)?;
        }
        Err(e) => {
             return Err(ContractError::Unauthorized {});
        }
        }
       
    }

    else{
        WHITEINFO.save(deps.storage, &user.address, &user)?;
    }
    Ok(Response::default())
}

fn execute_delete_user(
    deps: DepsMut,
    info: MessageInfo,
    user: String,
) -> Result<Response, ContractError> {
    let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }

    WHITEINFO.remove(deps.storage, &user);
    
    Ok(Response::default())
}





fn execute_chage_owner(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
   let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    CONFIG.update(deps.storage,
        |mut state|-> StdResult<_>{
            state.owner = address;
            Ok(state)
        }
    )?;
    Ok(Response::default())
}




fn execute_change_price(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
   let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    CONFIG.update(deps.storage,
        |mut state|-> StdResult<_>{
            state.price = amount;
            Ok(state)
        }
    )?;
    Ok(Response::default())
}

fn execute_set_nft_address(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
   let state =CONFIG.load(deps.storage)?;
    if state.owner != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }
    CONFIG.update(deps.storage,
        |mut state|-> StdResult<_>{
            state.nft_address = address;
            Ok(state)
        }
    )?;
    Ok(Response::default())
}



#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStateInfo {} => to_binary(& query_get_info(deps)?),
        QueryMsg::GetUserInfo { address }=>to_binary(& query_info(deps,address)?),
        QueryMsg::GetWhiteInfo { address }=>to_binary(& query_white_info(deps,address)?),
        QueryMsg::GetWhiteUsers {  }=>to_binary(& query_white_users(deps)?),
        QueryMsg::GetAdmin {  }=>to_binary(& query_admin_list(deps)?),
        QueryMsg::GetAdminInfo {address }=>to_binary(& query_admin_info(deps,address)?),
        QueryMsg::GetAllToken {  }=>to_binary(& query_all_token(deps)?),

    }
}


pub fn query_get_info(deps:Deps) -> StdResult<State>{
    let state = CONFIG.load(deps.storage)?;
    Ok(state)
}

pub fn query_info(deps:Deps,address:String) -> StdResult<Uint128>{
   let user_info = USERINFO.load(deps.storage, &address)?;
   Ok(user_info)
}


pub fn query_white_info(deps:Deps,address:String) -> StdResult<UserInfo>{
   let user_info = WHITEINFO.load(deps.storage, &address)?;
   Ok(user_info)
}


pub fn query_white_users(deps:Deps) -> StdResult<Vec<String>>{
   let users:StdResult<Vec<String>> = WHITEINFO
         .keys(deps.storage, None, None, Order::Ascending)
         .collect();
    Ok(users?)
}

pub fn query_admin_list(deps:Deps) -> StdResult<Vec<String>>{
   let admin:StdResult<Vec<String>> = ADMININFO
         .keys(deps.storage, None, None, Order::Ascending)
         .collect();
    Ok(admin?)
}

pub fn query_admin_info(deps:Deps,address:String) -> StdResult<UserInfo>{
   let admin_info = ADMININFO.load(deps.storage, &address)?;
   Ok(admin_info)
}



pub fn query_metadata(deps:Deps) -> StdResult<Vec<JunoFarmingMsg>>{
    let metadata = METADATA.load(deps.storage)?;
    Ok(metadata)
}


pub fn query_all_token(deps:Deps) -> StdResult<Vec<String>>{
    let ids = TOKENID.may_load(deps.storage)?;
    if ids ==None{
        let ids:Vec<String>  = vec![];
        Ok(ids)
    }
    else {
        Ok(ids.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{CosmosMsg};

    #[test]
    fn buy_token() {
        let mut deps = mock_dependencies();
        let instantiate_msg = InstantiateMsg {
            total_nft:Uint128::new(10),
            owner:"creator".to_string(),
            check_mint:vec![true,true,true,true,true],
            url :"url".to_string(),
            image_url:"imag_url".to_string(),
            price:Uint128::new(10),
            denom : "ujunox".to_string(),
            max_nft:Uint128::new(20)
        };
        let info = mock_info("creator", &[]);
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        

        let info = mock_info("creator", &[]);
        let message = ExecuteMsg::SetNftAddress { address:"nft_address1".to_string() };
        execute(deps.as_mut(), mock_env(), info, message).unwrap();

        let info = mock_info("creator", &[]);
        let message = ExecuteMsg::ChangePrice { amount:Uint128::new(12) };
        execute(deps.as_mut(), mock_env(), info, message).unwrap();

        let info = mock_info("creator", &[]);
        let message = ExecuteMsg::AddAdmin { admin : UserInfo { 
            address: "admin1".to_string(),
            email: Some("gmail.com".to_string()),
            first_name: Some("Jason".to_string()), 
            last_name: Some("Chen".to_string()),
            mobile : Some("12345".to_string()),
            contract_id:Some("contract".to_string()),
            build_type:Some("build".to_string()),
            role : Some("owner".to_string())
        } };
        execute(deps.as_mut(), mock_env(), info, message).unwrap();

        let admin = query_admin_list(deps.as_ref()).unwrap();
        assert_eq!(admin,vec!["admin1".to_string()]);

        let info = mock_info("admin1", &[]);
        let message = ExecuteMsg::AddWhiteUser { user : UserInfo { 
            address: "white1".to_string(),
            email: Some("gmail.com".to_string()),
            first_name: Some("Jason".to_string()), 
            last_name: Some("Chen".to_string()),
            mobile : Some("12345".to_string()),
            contract_id:Some("contract".to_string()),
            build_type:Some("build".to_string()),
            role : Some("owner".to_string())
        } };
        execute(deps.as_mut(), mock_env(), info, message).unwrap();

        let user = query_white_users(deps.as_ref()).unwrap();
        assert_eq!(user,vec!["white1".to_string()]);

        let state= query_get_info(deps.as_ref()).unwrap();
        assert_eq!(state.nft_address,"nft_address1".to_string());

     
       let info = mock_info("white1", &[Coin{
        denom:"ujunox".to_string(),
        amount:Uint128::new(12)
       }]);
       let message = ExecuteMsg::Mint { rand: Uint128::new(1) };
       let res = execute(deps.as_mut(), mock_env(), info, message).unwrap();
       assert_eq!(res.messages.len(),2);

       assert_eq!(res.messages[1].msg,CosmosMsg::Bank(BankMsg::Send {
                to_address: "creator".to_string(),
                amount:vec![Coin{
                    denom:"ujunox".to_string(),
                    amount:Uint128::new(12)
                }]
        }));
        
       let info = mock_info("creator", &[Coin{
        denom:"ujunox".to_string(),
        amount:Uint128::new(12)
       }]);
       let message = ExecuteMsg::Mint { rand: Uint128::new(2) };
       let res = execute(deps.as_mut(), mock_env(), info, message).unwrap();
       assert_eq!(res.messages.len(),1);

       let tokens = query_all_token(deps.as_ref()).unwrap();
       assert_eq!(tokens,vec!["LUNAR_NFT.1".to_string(),"LUNAR_NFT.2".to_string()]);
    }

}
