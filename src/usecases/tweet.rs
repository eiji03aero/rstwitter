use crate::domain::errors;
use crate::repositories;
use entity::tweet;

pub struct Usecase<'a> {
    tweets_repo: repositories::tweets::Repository<'a>,
}

impl<'a> Usecase<'a> {
    pub fn new(tweets_repo: repositories::tweets::Repository<'a>) -> Usecase<'a> {
        Self { tweets_repo }
    }

    pub async fn list(&self) -> Result<Vec<tweet::Model>, errors::DomainError> {
        match self.tweets_repo.list().await {
            Ok(users) => Ok(users),
            Err(e) => Err(e),
        }
    }

    pub async fn save(&self, user_id: i32, content: String) -> Result<(), errors::DomainError> {
        match self.tweets_repo.save(user_id, content).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
