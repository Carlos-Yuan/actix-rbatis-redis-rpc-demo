use macros::DTOOrderByCheck;
use rmp_serde as rmps;
use rpcx::{Error, ErrorKind, Result, RpcxParam, SerializeType};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(RpcxParam,Serialize, Deserialize,Default, Clone, Debug)]
pub struct UserTokenGetDTO {
    pub uid: u64,
    pub pf: u8,
}


#[derive(RpcxParam,Serialize, Deserialize,Default, Clone, Debug)]
pub struct UserTokenAddDTO {
    pub uid: u64,
    pub pf: u8,
    pub tk: String,
    pub openid: Option<String>,
    pub exp: u64,
}
