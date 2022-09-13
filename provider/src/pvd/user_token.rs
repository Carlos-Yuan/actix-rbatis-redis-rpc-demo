use crate::model::dto::{UserTokenGetDTO};
use crate::model::vo::{UserTokenJwtVO,UpdateTime};
use crate::model::{error::Result, table::UserToken};
use util::time;
use crate::{pool, dao::user_token};
use crate::{RPC, RT};
use rpcx::{RpcxParam};
use macros::rpcx_register;

pub struct UserTokenPvd;

#[rpcx_register]
impl UserTokenPvd{

    pub async fn get(args: UserTokenGetDTO) -> Result<UserToken> {
        let user_token = UserToken::get(pool!(), &args).await?;
        match user_token {
            Some(u) => Ok(u),
            None => Ok(UserToken::default()),
        }
    }

    pub async fn add(args: UserToken)->Result<UpdateTime>{
        let res = UserToken::insert(pool!(), &args).await?;
        if res.rows_affected>0{
            Ok(UpdateTime{time:args.u_time.unwrap()})
        }else{
            Ok(UpdateTime{time:0})
        }
    }

    pub async fn update(mut args: UserToken)->Result<UpdateTime>{
        let u_time=args.u_time.unwrap();
        args.u_time=Some(time::nanos());
        let res = UserToken::edit(pool!(), &args,&args.uid.unwrap(),&args.pf.unwrap(),&u_time).await?;
        if res.rows_affected>0{
            Ok(UpdateTime{time:args.u_time.unwrap()})
        }else{
            Ok(UpdateTime{time:0})
        }
    }

    pub async fn get_for_jwt(args: UserTokenGetDTO) -> Result<UserTokenJwtVO> {
        let res=user_token::get_for_jwt(pool!(),&args).await?;
        Ok(res)
    }
}



