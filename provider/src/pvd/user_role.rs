use crate::model::dto::{UserRoleGetDTO,EmptyDTO, UserModuleListDTO, UserModuleApiListDTO};
use crate::model::table::{UserModule, UserModuleApi};
use crate::model::vo::{UserModuleVO, UserModuleApiVO, UserModuleListVO, UserModuleApiListVO};
use crate::model::{error::Result, table::UserRole};
use crate::pool;
use crate::{RPC, RT};
use rpcx::{RpcxParam};
use macros::rpcx_register;

pub struct UserRolePvd;

#[rpcx_register]
impl UserRolePvd{

    pub async fn get(args: UserRoleGetDTO) -> Result<UserRole> {
        let user_role = UserRole::get(pool!(), &args.id).await?;
        println!("{:?}",user_role);
        match user_role {
            Some(u) => Ok(u),
            None => Ok(UserRole::default()),
        }
    }

    pub async fn get_all_model_api(args: EmptyDTO) -> Result<UserModuleListVO> {
        let modules = UserModule::list(pool!(),&UserModuleListDTO::default()).await?;
        let mids:Vec<u16>=modules.iter().map(|m|m.id.unwrap()).collect();
        let apis=UserModuleApi::get_by_mid_list(pool!(), &mids).await?;
        let modules=modules.iter().map(|m|{
            let mut mv=UserModuleVO::from(m);
            mv.apis=apis.iter().filter(|a|a.mid.clone().unwrap()==mv.id).
            map(|a|UserModuleApiVO::from(a)).collect();
            mv}).collect();
        Ok(UserModuleListVO{list:modules,total:0})
    }

    pub async fn list_api(args: UserModuleApiListDTO) -> Result<UserModuleApiListVO> {
        let apis=UserModuleApi::list(pool!(), &args).await?;
        let apis=apis.iter().map(|a|UserModuleApiVO::from(a)).collect();
        Ok(UserModuleApiListVO{list:apis,total:0})
    }
}



