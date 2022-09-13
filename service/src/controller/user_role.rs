use actix_web::{web, HttpResponse, Responder};
use macros::actix_config;

use crate::csm::UserRoleCsm;
use crate::model::dto::{UserRoleGetDTO,EmptyDTO, UserModuleApiListDTO};
use crate::model::vo::{UserRoleVO,Resp};
use crate::response;
use crate::csm::CSM;

pub struct UserRoleController;

#[actix_config]
impl UserRoleController {

    pub async fn get_all() -> impl Responder {
        let data = CSM.user_role
            .get_all_model_api(&EmptyDTO{})
            .await;
        response!(data)
    }

}
