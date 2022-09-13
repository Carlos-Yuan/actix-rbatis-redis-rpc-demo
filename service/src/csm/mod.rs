pub mod user;
pub mod user_role;
pub mod user_token;

pub use user::*;
pub use user_role::*;
pub use user_token::*;

use lazy_static::lazy_static;

pub struct Csm{
    pub user:UserCsm,
    pub user_role:UserRoleCsm,
    pub user_token:UserTokenCsm,
}

lazy_static!{
    pub static ref CSM:Csm = Csm{
        user:UserCsm::new(),
        user_role:UserRoleCsm::new(),
        user_token:UserTokenCsm::new(),
    };
}