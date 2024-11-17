use actix_web::dev::ServiceResponse;

pub fn custom_status_info(res: &ServiceResponse) -> &'static str {
    match res.status() {
        status if status.is_server_error() => "ERROR",
        status if status.is_client_error() => "WARN",
        status if status.is_success() => "INFO",
        _ => "-"
    }
}

