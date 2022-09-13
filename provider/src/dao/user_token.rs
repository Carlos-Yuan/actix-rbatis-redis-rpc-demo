
use rbatis::{impl_select,impl_insert, impl_update, Rbatis, py_sql};

use crate::model::{table::UserToken, dto::UserTokenGetDTO, vo::UserTokenJwtVO};

impl_insert!(UserToken{});

impl_select!(UserToken{get(dto:&UserTokenGetDTO)->Option=>
    "where uid = #{dto.uid} and pf=#{dto.pf}"});

impl_update!(UserToken{edit(uid:&u64,pf:&u8,u_time:&u64)=>
    "where uid = #{uid} and pf=#{pf} and u_time=#{u_time}"});

#[py_sql("select u.uid uid,ut.tk tk,ut.openid openid,ut.exp exp,ut.u_time u_time,ur.apis apis from user u LEFT JOIN user_token ut on u.role=ut.uid and ut.pf=#{dto.pf} LEFT JOIN user_role ur on u.role=ur.id where u.uid=#{dto.uid}")]
async fn get_for_jwt(rb: &Rbatis, dto:&UserTokenGetDTO) -> UserTokenJwtVO {}