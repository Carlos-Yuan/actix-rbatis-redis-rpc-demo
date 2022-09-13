use crate::model::Result;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use util::time;

/// JWT 鉴权 Token结构
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct JWTToken {
    //账号id
    pub uid: u64,
    //token
    pub tk: String,
    //登录平台
    pub pf: u8,
    //权限集合
    pub apis: Vec<u16>,
    //过期时间 用于刷新
    pub exp: u64,
}

impl JWTToken {
    /// create token
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err("JWTToken encode fail!".into()), // in practice you would return the error
        };
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken> {
        let validation = Validation::default();
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err("InvalidToken".into()), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => return Err("InvalidIssuer".into()), // Example on how to handle a specific error
                _ => return Err("InvalidToken other errors.into()".into()),
            },
        };
    }

    pub fn in_apis(&self,api:u16)->bool{
        for id in self.apis.iter(){
            if *id==api{
                return true
            }
        }
        false
    }

    pub fn is_exp(&self)->bool{
        if self.exp>time::millis(){
            false
        }else{
            true
        }
    }
}

