use rbatis::{ impl_select,impl_delete, impl_insert, impl_update};

use crate::{
     impl_select_page,
    model::{
        dto::{UserModuleApiListDTO, UserModuleListDTO},
        table::{UserModule, UserModuleApi, UserRole},
    },
};

impl_insert!(UserRole {});

impl_select!(UserRole{get(id:&u16)->Option=>"where id = #{id}"});

impl_update!(UserRole{edit(id:&u16,u_time:u64)=>"where id = #{id} and u_time=#{u_time}"});

impl_insert!(UserModule {});

impl_select!(UserModule{get(id:&u16)->Option=>"where id = #{id}"});

impl_select!(UserModule{list(dto:&UserModuleListDTO)=>
    "`where id!=-1`
      if dto.name!=null:
         ` and name = #{dto.name}`
      if dto.path!=null:
         ` and path = #{dto.path}`
         ` order by c_time `"});

impl_update!(UserModule{edit(id:&u16,u_time:u64)=>"where id = #{id} and u_time=#{u_time}"});

impl_delete!(UserModule{delete(id:&u16,u_time:u64)=>"where id = #{id} and u_time=#{u_time} and (select count(1) from user_module_api where mid=#{id})=0"});

impl_insert!(UserModuleApi {});

impl_select!(UserModuleApi{get(id:&u16)->Option=>"where id = #{id}"});

impl_select!(UserModuleApi{list(dto:&UserModuleApiListDTO)=>
    "where id!=-1 
    if dto.name!=null: 
        ` and name = #{dto.name}`
    if dto.path!=null:
        ` and path = #{dto.path}`
    if dto.mid!=null:
        ` and mid = #{dto.mid}`
    if dto.order_by!=null:
        ` order by ${dto.order_by}`
    if dto.page_on!=null:
        ` limit #{dto.page_on},#{dto.page_size}`
    "});

impl_select!(UserModuleApi{get_by_mid_list(mids:&Vec<u16>)=>
    "`where mid in (`
        trim ',': for _,v in mids:
           `#{v},`
    `)`"});

impl_update!(UserModuleApi{edit(id:&u16,u_time:u64)=>"where id = #{id} and u_time=#{u_time}"});

impl_delete!(UserModuleApi{delete(id:&u16,u_time:u64)=>"where id = #{id} and u_time=#{u_time}"});
