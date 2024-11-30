use actix_web::{http::header::ContentType, web, HttpResponse};
use uuid::Uuid;

use crate::{
    application::use_cases::user::UserUseCase,
    config::AppConfig,
    dto::{
        error::ErrorResponse,
        user_dto::{
            CreateRequest, CreateResponse, DeleteResponse, FindAllResponse, FindByIdResponse,
            UpdateRequest,
        },
    },
    infrastructure::repositories::postgres_user_repo::PostgresUserRepository,
};

use std::sync::Arc;

pub fn user_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(find_all_user))
            .route(web::post().to(create_new_user)),
    );
    cfg.service(
        web::resource("/{user_id}")
            .route(web::get().to(find_by_id))
            .route(web::put().to(update))
            .route(web::delete().to(delete)),
    );
}

async fn find_all_user(use_case: web::Data<UserUseCase<PostgresUserRepository>>) -> HttpResponse {
    match use_case.get_ref().find_all().await {
        Ok(users) => HttpResponse::Ok().json(FindAllResponse { data: users }),
        Err(err) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ErrorResponse {
                message: err.to_string(),
            }),
    }
}

async fn create_new_user(
    use_case: web::Data<UserUseCase<PostgresUserRepository>>,
    cfg: web::Data<Arc<AppConfig>>,
    req_body: web::Json<CreateRequest>,
) -> HttpResponse {
    let pwd_cfg = cfg.pwd.clone();
    let new_user = req_body.into_inner();

    match use_case.get_ref().create(new_user, pwd_cfg).await {
        Ok(user_id) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(CreateResponse { data: user_id }),
        Err(err) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ErrorResponse {
                message: err.to_string(),
            }),
    }
}

async fn find_by_id(
    use_case: web::Data<UserUseCase<PostgresUserRepository>>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = path.into_inner();
    match use_case.get_ref().find_by_id(user_id).await {
        Ok(Some(user)) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(FindByIdResponse { data: user }),
        Ok(None) => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json(ErrorResponse {
                message: "User Not Found".to_string(),
            }),
        Err(err) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ErrorResponse {
                message: err.to_string(),
            }),
    }
}

async fn update(
    use_case: web::Data<UserUseCase<PostgresUserRepository>>,
    path: web::Path<Uuid>,
    req_body: web::Json<UpdateRequest>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let update_data = req_body.into_inner();

    match use_case.get_ref().update(user_id, update_data).await {
        Ok(Some(user)) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(FindByIdResponse { data: user }),
        Ok(None) => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json(ErrorResponse {
                message: "User Not Found".to_string(),
            }),
        Err(err) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ErrorResponse {
                message: err.to_string(),
            }),
    }
}

async fn delete(
    use_case: web::Data<UserUseCase<PostgresUserRepository>>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = path.into_inner();

    match use_case.get_ref().delete(user_id).await {
        Ok(1_u64..) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(DeleteResponse {
                message: "User Deleted Successfully".to_string(),
            }),
        Ok(0) => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .json(ErrorResponse {
                message: "User Not Found".to_string(),
            }),
        Err(err) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ErrorResponse {
                message: err.to_string(),
            }),
    }
}
