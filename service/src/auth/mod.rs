use std::future::{ready, Ready};
use std::{fmt, rc::Rc};

use crate::csm::CSM;
use crate::model::dto::{UserModuleApiListDTO, UserTokenGetDTO};
use crate::model::table::UserToken;
use crate::model::vo::UserModuleApiVO;
use crate::CONF;
use crate::{model, REDIS};
use actix_web::body::BoxBody;
use actix_web::http::header::{self, HeaderValue, HeaderName};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use backtrace::Backtrace;
use futures_util::future::LocalBoxFuture;
use lazy_static::lazy_static;
use std::sync::RwLock;
use util::{rand, redis, time};

pub mod jwt;

lazy_static! {
    pub static ref ROLE_API: RwLock<RoleApi> = RwLock::new(RoleApi::default());
}

#[derive(Default, Clone)]
pub struct RoleApi {
    pub api: Vec<UserModuleApiVO>,
    pub exp: u64,
}

impl RoleApi {
    pub fn get_api_id(&self, path: String) -> u16 {
        for api in self.api.iter() {
            if api.path == path {
                return api.id;
            }
        }
        0
    }
}

fn get_api() -> RoleApi {
    let api = ROLE_API.read().unwrap();
    api.clone()
}

async fn set_api() -> RoleApi {
    let ref mut api = ROLE_API.write().unwrap();
    if api.exp > time::millis() {
        return api.clone();
    }
    let api_list = CSM
        .user_role
        .list_api(&UserModuleApiListDTO::default())
        .await;
    if let Ok(list) = api_list {
        api.api = list.list;
        api.exp = time::millis() + time::MINUTE5;
    } else {
        if api.exp == 0 {
            panic!("Find api error")
        } else if api.exp < time::millis() {
            api.exp = time::millis() + time::SECOND * 10;
        }
        println!("{:?}", api_list.err().unwrap());
    }
    api.clone()
}

static WHITE: &str = "/api/v1/User/login_by_phone|/api/v1/User/send_phone_code";

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;

impl<S: 'static> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    #[inline]
    fn poll_ready(
        &self,
        cx: &mut ::core::task::Context<'_>,
    ) -> ::core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Into::into)
    }

    fn call(&self,mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let mut pass = false;
            let path = req.path().to_string();
            let mut access_token = String::from("");
            let mut uid = 0u64;
            if WHITE.find(&path).is_some() {
                pass = true;
            } else {
                //todo auth 获得一个全局的请求方法的map值：编号，匹配后获得编号，确认token内包含该编号，判断token是否临近过期，则拉取redis锁刷新并下发token
                let mut apis = get_api();
                if apis.exp < time::millis() {
                    apis = set_api().await;
                }
                let api_id = apis.get_api_id(path);
                if api_id > 0 {
                    let token = req
                        .headers()
                        .get(header::AUTHORIZATION)
                        .map(|v| v.to_str().unwrap_or_default().to_string())
                        .unwrap_or_default();
                    let j = jwt::JWTToken::verify(CONF.secret.as_str(), token.as_str());
                    if j.is_ok() {
                        let j = j.unwrap();
                        if j.in_apis(api_id) {
                            if j.is_exp() {
                                match check_token(j).await {
                                    Ok(jwt) => {
                                        access_token =jwt.create_token(CONF.secret.as_str()).unwrap();
                                        pass = true;
                                        uid=jwt.uid;
                                    }
                                    Err(err) => return Err(e401(err)),
                                }
                            } else {
                                pass = true;
                                uid=j.uid;
                            }
                        }
                    }
                }
            }
            if pass {
                if uid!=0{
                    req.headers_mut().insert(HeaderName::from_lowercase(b"user").unwrap(),HeaderValue::from_str(uid.to_string().as_str()).unwrap());
                }
                let mut res = svc.call(req).await?;
                if access_token != "" {
                    res.headers_mut().insert(
                        header::AUTHORIZATION,
                        HeaderValue::from_str(access_token.as_str()).unwrap(),
                    );
                }
                Ok(res)
            } else {
                Err(e401("Not auth"))
            }
        })
    }
}

fn e401<E: fmt::Debug + fmt::Display + 'static>(err: E) -> actix_web::Error {
    actix_web::error::InternalError::from_response(err, HttpResponse::Unauthorized().finish())
        .into()
}

async fn check_token(mut j: jwt::JWTToken) -> model::Result<jwt::JWTToken> {
    let ref res = CSM
        .user_token
        .get_for_jwt(&UserTokenGetDTO {
            uid: j.uid,
            pf: j.pf,
        })
        .await?;
    //token过期 需检查缓存 更新token是否与现token一致
    if res.tk != j.tk {
        let f_tk: String = REDIS.get(format!("front:tk:{}", res.tk)).await?;
        if f_tk != j.tk {
            return Err("token expire".into());
        }
        j.tk = res.tk.clone();
    } else {
        //刷新user_token 默认是30天过期，每天更新一次
        if res.exp < time::millis() + time::MONTH - time::DAY {
            if REDIS
                .lock(
                    format!("user:token:refresh:{}", j.uid).as_str(),
                    redis::MINUTE10,
                )
                .await?
            {
                let user_tk = UserToken {
                    uid: Some(res.uid),
                    pf: Some(j.pf),
                    tk: Some(rand::str(32)),
                    openid: None,
                    exp: Some(time::millis() + time::MONTH),
                    c_time: None,
                    u_time: Some(res.u_time),
                };
                let utime = CSM.user_token.update(&user_tk).await?;
                if utime.time == 0 {
                    return Err("token expire".into());
                }
                j.tk = user_tk.tk.unwrap();
                REDIS
                    .set(format!("front:tk:{}", res.tk), j.tk.clone(), redis::MINUTE)
                    .await?;
            }
        }
    }
    j.apis = res.to_vec_apis();
    j.exp = time::millis() + time::MINUTE;
    Ok(j)
}
