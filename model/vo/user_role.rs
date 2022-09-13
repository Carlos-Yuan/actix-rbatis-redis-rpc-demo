use rmp_serde as rmps;
use rpcx::{Error, ErrorKind, Result, RpcxParam, SerializeType};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use crate::model::{table::{UserRole, UserModuleApi, UserModule}};


#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleVO {
    pub id: u16,
    pub name: String,
    pub apis: String,
    pub c_time: u64,
    pub u_time: u64
}

impl From<UserRole> for UserRoleVO {
    fn from(data: UserRole) -> Self {
        UserRoleVO {
            id: data.id.unwrap(),
            name: data.name.unwrap(),
            apis: data.apis.unwrap(),
            c_time: data.c_time.unwrap(),
            u_time: data.u_time.unwrap(),
        }
    }
}

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModuleVO {
    pub id: u16,
    pub name: String,
    pub c_time: u64,
    pub u_time: u64,
    pub apis:Vec<UserModuleApiVO>,
}

impl From<&UserModule> for UserModuleVO {
    fn from(data: &UserModule) -> Self {
        UserModuleVO {
            id: data.id.unwrap(),
            name: data.name.clone().unwrap(),
            c_time: data.c_time.unwrap(),
            u_time: data.u_time.unwrap(),
            apis:vec![]
        }
    }
}

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserModuleListVO {
    pub list: Vec<UserModuleVO>,
    pub total: u64,
}

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModuleApiVO {
    pub id: u16,
    pub path: String,
    pub name: String,
    pub mid: u16,
    pub c_time: u64,
    pub u_time: u64
}

impl From<&UserModuleApi> for UserModuleApiVO {
    fn from(data: &UserModuleApi) -> Self {
        UserModuleApiVO {
            id: data.id.unwrap(),
            path: data.path.clone().unwrap(),
            name: data.name.clone().unwrap(),
            mid: data.mid.unwrap(),
            c_time: data.c_time.unwrap(),
            u_time: data.u_time.unwrap(),
        }
    }
}

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserModuleApiListVO {
    pub list: Vec<UserModuleApiVO>,
    pub total: u64,
}
