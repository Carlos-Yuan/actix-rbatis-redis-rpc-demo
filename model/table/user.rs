use rmp_serde as rmps;
use rpcx::{Error, ErrorKind, Result, RpcxParam, SerializeType};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uid: Option<u64>,
    pub nickname: Option<String>,
    pub phone: Option<String>,
    pub name: Option<String>,
    pub pwd: Option<String>,
    pub role: Option<u16>,
    pub c_time: Option<u64>,
    pub u_time: u64
}

impl User{
    pub fn is_none(&self)->bool{
        self.uid.is_none()
    }
}
