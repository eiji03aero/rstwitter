use sea_orm::DbErr;

use crate::domain::errors;
use crate::repositories;

pub struct Usecase<'a> {
    users_repo: repositories::users::Repository<'a>,
}

impl<'a> Usecase<'a> {
    pub fn new(users_repo: repositories::users::Repository<'a>) -> Usecase<'a> {
        Self { users_repo }
    }

    pub async fn save(&self, username: &str) -> Result<(), errors::DomainError> {
        match self.users_repo.save(username).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
