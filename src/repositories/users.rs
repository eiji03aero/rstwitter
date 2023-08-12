use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, NotSet, Set};

use crate::domain::errors;
use entity::user;

pub struct Repository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> Repository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Repository<'a> {
        Self { db }
    }

    pub async fn list(&self) -> Result<Vec<user::Model>, errors::DomainError> {
        match user::Entity::find().all(self.db).await {
            Ok(users) => Ok(users),
            Err(_) => Err(errors::UnknownError::new()),
        }
    }

    pub async fn save(&self, username: &str) -> Result<(), errors::DomainError> {
        let now = Utc::now().naive_utc();

        let user_am = user::ActiveModel {
            id: NotSet,
            username: Set(username.to_owned()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        match user_am.insert(self.db).await {
            Ok(_) => Ok(()),
            Err(_) => Err(errors::UnknownError::new()),
        }
    }
}
