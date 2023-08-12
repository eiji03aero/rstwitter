use actix_web::{get, post, web, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::handlers;
use crate::repositories;
use crate::usecases;

#[derive(Serialize)]
pub struct ListModel {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}
#[derive(Serialize)]
pub struct ListResponseBody {
    pub tweets: Vec<ListModel>,
}
#[get("/tweets")]
pub async fn list(
    data: web::Data<handlers::AppState>,
) -> Result<impl Responder, handlers::ApiError> {
    let db = &data.db;
    let tweet_usecase = usecases::tweet::Usecase::new(repositories::tweets::Repository::new(db));

    match tweet_usecase.list().await {
        Ok(tweets) => {
            let list = tweets
                .into_iter()
                .map(|t| ListModel {
                    id: t.id,
                    user_id: t.user_id,
                    content: t.content,
                    created_at: t.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                    updated_at: t.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                })
                .collect();
            let response = ListResponseBody { tweets: list };
            Ok(web::Json(response))
        }
        Err(_) => Err(handlers::ApiError::InternalError),
    }
}

#[derive(Deserialize)]
pub struct CreateRequestBody {
    user_id: i32,
    content: String,
}
#[derive(Serialize)]
pub struct CreateResponseBody {
    message: String,
}
#[post("/tweets")]
pub async fn create(
    data: web::Data<handlers::AppState>,
    body: web::Json<CreateRequestBody>,
) -> Result<impl Responder, handlers::ApiError> {
    let db = &data.db;
    let tweet_usecase = usecases::tweet::Usecase::new(repositories::tweets::Repository::new(db));

    match tweet_usecase
        .save(body.user_id, body.content.to_string())
        .await
    {
        Ok(_) => {
            let response = CreateResponseBody {
                message: String::from("Created tweet"),
            };
            Ok(web::Json(response))
        }
        Err(_) => Err(handlers::ApiError::InternalError),
    }
}
