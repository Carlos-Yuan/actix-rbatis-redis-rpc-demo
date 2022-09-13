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

#[derive(RpcxParam, Default, Debug, Clone, Serialize, Deserialize)]
pub struct EmptyDTO{}

#[derive(RpcxParam,Serialize, Deserialize,Default, Clone, Debug)]
pub struct PagingDTO{
    pub order: Option<String>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}