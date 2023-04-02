use chrono;
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet, Set};

use crate::domain::errors;
use entity::user;

pub struct Repository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> Repository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Repository<'a> {
        Self { db }
    }

    pub async fn save(&self, username: &str) -> Result<(), errors::DomainError> {
        let user_am = user::ActiveModel {
            id: NotSet,
            username: Set(username.to_owned()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        };

        match user_am.insert(self.db).await {
            Ok(_) => Ok(()),
            Err(_) => Err(errors::UnknownError::new()),
        }
    }
}
