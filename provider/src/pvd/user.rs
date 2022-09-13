use crate::model::dto::user::UserGetDTO;
use crate::model::{error::Result, table::user::User};
use crate::pool;
use crate::{RPC, RT};
use rpcx::{RpcxParam};
use macros::rpcx_register;

pub struct UserPvd;

#[rpcx_register]
impl UserPvd{
    pub async fn get(args: UserGetDTO) -> Result<User> {
        let user = User::get(pool!(), &args).await?;
        match user {
            Some(u) => Ok(u),
            None => Ok(User::default()),
        }
    }
}



