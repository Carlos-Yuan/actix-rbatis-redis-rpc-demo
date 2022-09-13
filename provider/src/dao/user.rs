
use rbatis::{impl_select,impl_insert, impl_update};

use crate::model::{table::User, dto::UserGetDTO};

impl_insert!(User{});

impl_select!(User{get(dto:&UserGetDTO)->Option=>
    "`where uid!=-1`
      if dto.uid!=null:
         ` and uid = #{dto.uid}`
      if dto.phone!=null:
         ` and phone = #{dto.phone}`
      "});

impl_update!(User{edit(uid:&u64,u_time:u64)=>
    "where uid = #{uid} and u_time=#{u_time}"});