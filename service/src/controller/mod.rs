pub mod user;
pub mod user_role;

pub use user::*;
pub use user_role::*;

#[macro_export]
macro_rules! response {
    ($data:expr) => {
        match $data {
            Result::Err(e) => HttpResponse::Ok().json(Resp::set_err(e)),
            Result::Ok(r) => HttpResponse::Ok().json(Resp::set_data(r)),
        }
    };
    ($data:expr,$return_vo_type:ty) => {
        match $data {
            Result::Err(e) => HttpResponse::Ok().json(Resp::set_err(e)),
            Result::Ok(r) => HttpResponse::Ok().json(Resp::set_data(<$return_vo_type>::from(r))),
        }
    };
}

#[macro_export]
macro_rules! response_err {
    ($err:expr) => {
        HttpResponse::Ok().json(Resp::set_err($err))
    };
}

#[macro_export]
macro_rules! response_msg {
    ($err:expr) => {
        HttpResponse::Ok().json(Resp::set_msg($err))
    };
}

#[macro_export]
macro_rules! response_ok {
    () => {
        HttpResponse::Ok().json(Resp::<()>::ok())
    };
}

#[macro_export]
macro_rules! response_obj {
    ($data:expr) => {
        HttpResponse::Ok().json(Resp::set_data($data))
    };
}

#[macro_export]
macro_rules! get_header {
    ($data:expr,$v_type:ty,$req:expr) => {
        $req.headers().get($data).unwrap().to_str().unwrap().parse::<$v_type>().unwrap()
    };
}
