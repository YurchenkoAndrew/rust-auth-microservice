use actix_web::{web, middleware::{NormalizePath, TrailingSlash}};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("")
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            // .service(register)
    );
}