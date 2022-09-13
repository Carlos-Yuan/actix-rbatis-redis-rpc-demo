pub mod user;
pub mod user_role;
pub mod user_token;

pub fn register() {
   user::UserPvd::register();
   user_role::UserRolePvd::register();
   user_token::UserTokenPvd::register();
}
