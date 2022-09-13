use macros::DTOOrderByCheck;
use rmp_serde as rmps;
use rpcx::{Error, ErrorKind, Result, RpcxParam, SerializeType};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(RpcxParam,Serialize, Deserialize,Default, Clone, Debug)]
pub struct UserRoleGetDTO {
    pub id: u16,
}


#[derive(RpcxParam,Serialize, Deserialize,Default, Clone, Debug)]
pub struct UserModuleListDTO {
    pub name: Option<String>,
    pub path: Option<String>,
    pub order_by: Option<String>,
    pub page_on: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(RpcxParam,DTOOrderByCheck,Serialize, Deserialize,Default, Clone, Debug)]
pub struct UserModuleApiListDTO {
    pub name: Option<String>,
    pub path: Option<String>,
    pub mid: Option<u16>,
    pub order_by: Option<String>,
    pub page_on: Option<u64>,
    pub page_size: Option<u64>,
}
