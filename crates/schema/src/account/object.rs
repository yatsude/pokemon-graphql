use async_graphql::Object;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use shared::scalars::Email;

#[derive(Default)]
pub struct Account {
    pub id: Uuid,
    pub email: Email,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[Object]
impl Account {
    async fn id(&self) -> Uuid {
        self.id
    }

    async fn email(&self) -> Email {
        self.email.clone()
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
