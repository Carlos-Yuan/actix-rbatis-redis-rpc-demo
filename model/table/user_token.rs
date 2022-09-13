use rmp_serde as rmps;
use rpcx::{Error, ErrorKind, Result, RpcxParam, SerializeType};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserToken {
    pub uid: Option<u64>,
    pub pf: Option<u8>,
    pub tk: Option<String>,
    pub openid: Option<String>,
    pub exp: Option<u64>,
    pub c_time: Option<u64>,
    pub u_time: Option<u64>
}

const PF_APP:u8=1;

impl UserToken{
    pub fn is_none(&self)->bool{
        self.uid.is_none()
    }
}