use rmp_serde as rmps;
use rpcx::{Error, ErrorKind, Result, RpcxParam, SerializeType};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(Serialize, Deserialize,Default, Clone, Debug)]
pub struct SendPhoneCodeDTO {
    pub phone: String,
}

#[derive(Serialize, Deserialize,Default, Clone, Debug)]
pub struct UserLoginByPhoneDTO {
    pub phone: String,
    pub code: String,
    pub pf: u8,
}


#[derive(RpcxParam,Serialize, Deserialize,Default, Clone, Debug)]
pub struct UserGetDTO {
    pub uid: Option<u64>,
    pub phone:Option<String>,
}