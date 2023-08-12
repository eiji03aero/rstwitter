use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, NotSet, Set};

use crate::domain::errors;
use entity::tweet;

pub struct Repository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> Repository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Repository<'a> {
        Self { db }
    }

    pub async fn list(&self) -> Result<Vec<tweet::Model>, errors::DomainError> {
        match tweet::Entity::find().all(self.db).await {
            Ok(tweets) => Ok(tweets),
            Err(_) => Err(errors::UnknownError::new()),
        }
    }

    pub async fn save(
        &self,
        user_id: i32,
        content: String,
    ) -> Result<tweet::Model, errors::DomainError> {
        let now = Utc::now().naive_utc();

        let tweet_am = tweet::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            content: Set(content),
            created_at: Set(now),
            updated_at: Set(now),
        };

        match tweet_am.insert(self.db).await {
            Ok(tweet) => Ok(tweet),
            Err(_) => Err(errors::UnknownError::new()),
        }
    }
}
