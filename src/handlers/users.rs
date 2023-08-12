use actix_web::{get, post, web, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::handlers;
use crate::repositories;
use crate::usecases;

#[derive(Serialize)]
pub struct ListModel {
    pub id: i32,
    pub username: String,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Serialize)]
pub struct ListResponseBody {
    pub users: Vec<ListModel>,
}
#[get("/users")]
pub async fn list(
    data: web::Data<handlers::AppState>,
) -> Result<impl Responder, handlers::ApiError> {
    let db = &data.db;
    let user_usecase = usecases::user::Usecase::new(repositories::users::Repository::new(db));

    match user_usecase.list().await {
        Ok(users) => {
            let list = users
                .into_iter()
                .map(|u| ListModel {
                    id: u.id,
                    username: u.username,
                    created_at: u.created_at.format("%Y-%m-%s %H:%M:%S").to_string(),
                    updated_at: u.updated_at.format("%Y-%m-%s %H:%M:%S").to_string(),
                })
                .collect();
            let response = ListResponseBody { users: list };
            Ok(web::Json(response))
        }
        Err(_) => Err(handlers::ApiError::InternalError),
    }
}

#[derive(Deserialize)]
pub struct CreateRequestBody {
    username: String,
}
#[derive(Serialize)]
pub struct CreateResponseBody {
    message: String,
}
#[post("/users")]
pub async fn create(
    data: web::Data<handlers::AppState>,
    body: web::Json<CreateRequestBody>,
) -> Result<impl Responder, handlers::ApiError> {
    let db = &data.db;
    let user_usecase = usecases::user::Usecase::new(repositories::users::Repository::new(db));

    match user_usecase.save(body.username.as_str()).await {
        Ok(_) => {
            let response = CreateResponseBody {
                message: String::from("Created user"),
            };
            Ok(web::Json(response))
        }
        Err(_) => Err(handlers::ApiError::InternalError),
    }
}
