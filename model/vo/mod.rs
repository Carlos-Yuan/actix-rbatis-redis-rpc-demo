use rmp_serde as rmps;
use rpcx::{Error, ErrorKind, Result, RpcxParam, SerializeType};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

pub mod user;
pub mod user_role;
pub mod user_token;

pub use user::*;
pub use user_role::*;
pub use user_token::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resp<T>{
    pub status:bool,
    pub code:u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data:Option<T>
}

impl<T> Resp<T>{
    pub fn set_data(data:T)->Self{
        Resp{status:true,code:0, data: Some(data) }
    }

    pub fn set_msg(data:T)->Self{
        Resp{status:false,code:1, data: Some(data) }
    }

    pub fn set_err(err:T)->Self{
        Resp{status:false,code:2,data:Some(err)}
    }

    pub fn ok()->Self{
        Resp{status:true,code:0,data:None}
    }
}

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTime{
    pub time:u64
}