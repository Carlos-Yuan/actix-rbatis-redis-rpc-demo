use macros::{rpcx_call,CsmNew};
use rpcx::{Client};
use crate::model::{table::UserToken, error::Result,dto::UserTokenGetDTO,vo::{UserTokenJwtVO,UpdateTime}};
use crate::CONF;

#[derive(CsmNew)]
pub struct UserTokenCsm{
    client:Client,
}

#[rpcx_call]
impl UserTokenCsm{

    pub async fn get(&self,args:&UserTokenGetDTO)->Result<UserToken>{}

    pub async fn add(&self,args:&UserToken)->Result<UpdateTime>{}

    pub async fn update(&self,args:&UserToken)->Result<UpdateTime>{}

    pub async fn get_for_jwt(&self,args:&UserTokenGetDTO) -> Result<UserTokenJwtVO> {}
}

