mod config;
mod dao;
mod model;
mod pvd;

use std::{thread, cell::RefCell, sync::Mutex};

use config::Config;
use rbatis::rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use lazy_static::lazy_static;
use util::redis::RedisConnection;
use rpcx::Server;

lazy_static! {
    pub static ref CONF:Config = config::load();
    pub static ref RBATIS:Rbatis = Rbatis::new();
    pub static ref REDIS:RedisConnection =RedisConnection::new(CONF.redis.clone());
    pub static ref RPC: Mutex<RefCell<Server>> = Mutex::new(RefCell::new(Server::new(CONF.rpcx_addr(),0)));
    pub static ref RT: tokio::runtime::Runtime=thread::spawn(|| {
        tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap()
    }).join().expect("create runtime panicked");
}

fn main() {
    RT.block_on(async{
        fast_log::init(fast_log::config::Config::new().console()).expect("logger init failed");
        log::info!("initiated logging");
        link_db().await;
        log::info!("initiated rbatis");
        pvd::register();
        log::info!("rpc registered");
        RPC.lock().unwrap().borrow_mut().start().unwrap();
    });
}


pub async fn link_db() {
    //连接数据库
    RBATIS
        .link(MysqlDriver {}, &CONF.database.assemble().as_str())
        .await
        .expect("rbatis pool init fail!");
    println!("rbatis pool init success! pool state = {:?}", RBATIS.get_pool().expect("pool not init!").inner.status());
}

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::RBATIS.clone()
    };
}