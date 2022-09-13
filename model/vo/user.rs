use rmp_serde as rmps;
use rpcx::{Error, ErrorKind, Result, RpcxParam, SerializeType};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use crate::model::{table::User};


#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowUserVO {
    pub uid: u64,
    pub nickname:String,
    pub name: String,
    pub role: u16,
    pub c_time:u64,
}

impl From<User> for ShowUserVO {
    fn from(user: User) -> Self {
        ShowUserVO {
            uid: user.uid.unwrap(),
            name: user.name.unwrap(),
            nickname: user.nickname.unwrap(),
            role:user.role.unwrap(),
            c_time: user.c_time.unwrap(),
        }
    }
}