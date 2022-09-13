use macros::{rpcx_call,CsmNew};
use rpcx::{Client};
use crate::model::{table::UserRole, error::Result,dto::{UserRoleGetDTO,EmptyDTO,UserModuleApiListDTO},vo::{UserModuleListVO,UserModuleApiListVO}};
use crate::CONF;

#[derive(CsmNew)]
pub struct UserRoleCsm{
    client:Client,
}

#[rpcx_call]
impl UserRoleCsm{

    pub async fn get(&self,args:&UserRoleGetDTO)->Result<UserRole>{}

    pub fn get_all_model_api(&self,args:&EmptyDTO) -> Result<UserModuleListVO> {}

    pub async fn list_api(&self,args:&UserModuleApiListDTO) -> Result<UserModuleApiListVO> {
        args.check_order_by()?
    }
}

