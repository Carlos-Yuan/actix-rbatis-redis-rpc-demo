
use std::sync::{Arc, RwLock};

use backtrace::Backtrace;
use futures::prelude::*;
use redis_cluster_async::{Client,redis::{cmd, Script, self}};

struct C{
  pub a:u64
}

#[tokio::main]
async fn main() {

  let c=RwLock::new(C{a:1});
  {
    let mut c=c.write().unwrap();
    c.a=2;
  }
  let c1=c.read().unwrap();
  println!("{}",c1.a);



    // let redis_uri = vec!["redis://:ccc7a7ad90880ab6b81cd2f384638c58@127.0.0.1:6381/"];


    // let client = Client::open(redis_uri).unwrap();

    // let mut connection = client.get_connection().await.unwrap();
    // let res: u32 = Script::new(
    //     r#"local times = redis.call('incr',KEYS[1])
    //     if times == 1 then
    //       redis.call('expire',KEYS[1], ARGV[1])
    //     end
    //      if times > tonumber(ARGV[2]) then
    //        return 0
    //      end
    //     return 1"#,
    // )
    // .key("user:login:smsit:18888")
    // .arg(1000)
    // .arg(2)
    // .invoke_async(&mut connection)
    // .await.unwrap();
    //     println!("{}",res);
    //     // let s="local times = redis.call('incr',KEYS[1])
    //     // return times";
    //     // let val:u32=redis::cmd("EVAL").arg(s).arg(1).arg("{user:login:sms:limit:18888}").query(&mut connection).unwrap();

    //     let val:u32=redis::cmd("GET").arg("{user:login:sm22s:limit:18888}").query_async(&mut connection).await.unwrap();
    //     // println!("{}",val);
}