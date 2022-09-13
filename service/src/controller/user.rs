use actix_web::{web, HttpResponse, Responder, HttpRequest};
use macros::actix_config;
use util::redis::RedisUtilError;

use crate::model::dto::{UserLoginByPhoneDTO, SendPhoneCodeDTO, UserTokenGetDTO, UserRoleGetDTO, EmptyDTO};
use crate::model::dto::user::UserGetDTO;
use crate::model::table::UserToken;
use crate::model::vo::{user::ShowUserVO,Resp};
use crate::{response, response_err, response_msg, response_ok, response_obj, CONF, get_header};
use crate::REDIS;
use util::{redis, rand, time};

use crate::csm::{CSM};

use crate::auth::jwt;

pub struct UserController;

#[actix_config]
impl UserController {
    
    pub async fn get(req: HttpRequest) -> impl Responder {
        let uid = get_header!("user",u64,req);
        let data = CSM.user
            .get(&UserGetDTO { uid: Some(uid), phone: None })
            .await;
        response!(data,ShowUserVO)
    }

    pub async fn send_phone_code(args: web::Json<SendPhoneCodeDTO>) -> impl Responder {
        let limit=REDIS.limit(format!("{}{}","user:login:sms:limit:",args.phone).as_str(), 55, 1).await;
        if limit.is_err(){
            return response_err!(limit.err());
        }
        if !limit.unwrap(){
            return response_msg!("发送过于频繁");
        }
        let code=rand::number_str("",5);
        let set=REDIS.set(format!("{}{}","user:login:phone:code:",args.phone), code.clone(), redis::MINUTE5).await;
        if set.is_err(){
            return response_err!(set.err());
        }
        response_obj!(code)
    }

    pub async fn login_by_phone(args: web::Json<UserLoginByPhoneDTO>) -> impl Responder {
        let code_key=format!("{}{}","user:login:phone:code:",args.phone);
        let code:Result<String,RedisUtilError>=REDIS.get(code_key).await;
        if code.is_err(){
            let err=code.err().unwrap();
            if let RedisUtilError::RedisNil = err{
                return response_msg!("验证码已过期");
            }
            return response_err!(err);
        }
        let code=code.unwrap();
        if code==args.code{
            let user=CSM.user.get(&UserGetDTO { uid: None, phone: Some(args.phone.clone()) }).await;
            if user.is_err(){
                return response_err!(user.err());
            }
            let user=user.unwrap();
            if !user.is_none(){
                let ut=CSM.user_token.get(&UserTokenGetDTO{uid:user.uid.unwrap(),pf:args.pf}).await;
                if ut.is_err(){
                    return response_err!(ut.err().unwrap());
                }
                let mut ut=ut.unwrap();
                if ut.is_none(){
                    ut=UserToken{uid:user.uid, pf: Some(args.pf), tk: Some(rand::str(32)), openid: Some("".to_string()), exp: Some(time::millis()+time::MONTH), c_time:Some(time::millis()), u_time: Some(time::nanos()) };
                    let utime=CSM.user_token.add(&ut).await;
                    if utime.is_err(){
                        return response_err!(utime.err().unwrap());
                    }
                    if utime.unwrap().time==0{
                        return response_msg!("设置登录信息失败")
                    }
                }else{
                    ut.tk=Some(rand::str(32));
                    let utime=CSM.user_token.update(&ut).await;
                    if utime.is_err(){
                        return response_err!(utime.err().unwrap());
                    }
                    if utime.unwrap().time==0{
                        return response_msg!("设置登录信息失败")
                    }
                }
                let role=CSM.user_role.get(&UserRoleGetDTO{ id: user.role.unwrap() }).await;
                if role.is_err(){
                    return response_err!(role.err().unwrap());
                }
                let tk=jwt::JWTToken{ uid: user.uid.unwrap(), tk: ut.tk.unwrap(), pf: args.pf, apis: role.unwrap().to_vec_apis(), exp: time::millis()+time::SECOND30 }.create_token(CONF.secret.as_str());
                if tk.is_err(){
                    return response_err!(tk.err().unwrap());
                }
                HttpResponse::Ok().insert_header(("authorization", tk.unwrap())).json(Resp::<()>::ok())
            }else{
                response_msg!("用户不存在")
            }
        }else{
            response_msg!("验证码有误")
        }
    }
}
