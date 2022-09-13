use rmp_serde as rmps;
use rpcx::{Error, ErrorKind, Result, RpcxParam, SerializeType};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use crate::model::{table::{UserToken}};


#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTokenVO {
    pub uid: u64,
    pub pf: u8,
    pub tk: String,
    pub openid: String,
    pub exp: u64,
    pub c_time: u64,
    pub u_time: u64,
}

impl From<UserToken> for UserTokenVO {
    fn from(data: UserToken) -> Self {
        UserTokenVO {
            uid: data.uid.unwrap(),
            pf: data.pf.unwrap(),
            tk: data.tk.unwrap(),
            openid: data.openid.unwrap(),
            exp: data.exp.unwrap(),
            c_time: data.c_time.unwrap(),
            u_time: data.u_time.unwrap(),
        }
    }
}

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserTokenJwtVO {
    pub uid: u64,
    pub tk: String,
    pub openid: String,
    pub exp: u64,
    pub apis: String,
    pub u_time: u64,
}

impl UserTokenJwtVO{
    pub fn to_vec_apis(&self)->Vec<u16>{
        serde_json::from_str(self.apis.as_str()).unwrap()
    }
}