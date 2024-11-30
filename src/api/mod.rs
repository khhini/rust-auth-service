pub mod health_check;
pub mod user;

use crate::api::health_check::health_check_cfg;
use crate::application::use_cases::user::UserUseCase;
use crate::infrastructure::repositories::postgres_user_repo::PostgresUserRepository;
use actix_web::web;
use sqlx::PgPool;

use self::user::user_cfg;

pub fn api_v1_cfg(cfg: &mut web::ServiceConfig, pool: PgPool) {
    cfg.service(web::scope("/healthz").configure(health_check_cfg));

    let user_repository = PostgresUserRepository::new(pool);
    let user_use_case = UserUseCase::new(user_repository);
    cfg.service(
        web::scope("/users")
            .app_data(web::Data::new(user_use_case))
            .configure(user_cfg),
    );
}
