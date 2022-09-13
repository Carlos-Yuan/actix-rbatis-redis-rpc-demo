use rmp_serde as rmps;
use rpcx::{Error, ErrorKind, Result, RpcxParam, SerializeType};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub id: Option<u16>,
    pub name: Option<String>,
    pub apis: Option<String>,
    pub c_time: Option<u64>,
    pub u_time: Option<u64>
}
impl UserRole{
    pub fn to_vec_apis(&self)->Vec<u16>{
        serde_json::from_str(self.apis.clone().unwrap().as_str()).unwrap()
    }
}


#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserModule {
    pub id: Option<u16>,
    pub name: Option<String>,
    pub c_time: Option<u64>,
    pub u_time: Option<u64>
}

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserModuleApi {
    pub id: Option<u16>,
    pub path: Option<String>,
    pub name: Option<String>,
    pub mid: Option<u16>,
    pub c_time: Option<u64>,
    pub u_time: Option<u64>,
}