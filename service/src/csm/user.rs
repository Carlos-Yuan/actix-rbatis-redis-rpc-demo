use macros::{rpcx_call,CsmNew};
use rpcx::{Client};
use crate::model::{table::User, error::Result,dto::user::UserGetDTO};
use crate::CONF;

#[derive(CsmNew)]
pub struct UserCsm{
    client:Client,
}

#[rpcx_call]
impl UserCsm{

    pub async fn get(&self,args:&UserGetDTO)->Result<User>{}
}

