use actix_web::{post, web, HttpResponse, Responder, Result};
use serde::Deserialize;

use crate::handlers;
use crate::repositories;
use crate::usecases;

#[derive(Deserialize)]
pub struct CreateBody {
    username: String,
}
#[post("/users")]
pub async fn create(
    data: web::Data<handlers::AppState>,
    body: web::Json<CreateBody>,
) -> Result<impl Responder, handlers::ApiError> {
    let db = &data.db;
    let user_usecase = usecases::user::Usecase::new(repositories::users::Repository::new(db));

    match user_usecase.save(body.username.as_str()).await {
        Ok(_) => Ok(HttpResponse::Created()),
        Err(_) => Err(handlers::ApiError::InternalError),
    }
}
