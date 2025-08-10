use actix_multipart::form::MultipartFormConfig;
use actix_web::web::FormConfig;
use actix_web::{error, web, HttpResponse};

pub fn multipart_form_config() -> MultipartFormConfig {
    
    MultipartFormConfig::default().memory_limit(40_000 * 1_000 * 1_000)
}

pub fn json_form_config() -> FormConfig {
    
    web::FormConfig::default()
        .limit(40000 * 1000 * 1000)
        .error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        })
}
