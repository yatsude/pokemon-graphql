use async_graphql::*;
use sqlx::PgPool;
use uuid::Uuid;

use shared::scalars::Email;

use crate::account::input::RegisterInput;
use crate::account::Account;

#[tracing::instrument(name = "get one account from database", skip(conn))]
pub async fn get_account_by_email(conn: &PgPool, email: Email) -> Result<Account, Error> {
    let res = sqlx::query_as!(
        Account,
        r#"
            SELECT id, email, name, created_at FROM account WHERE email = $1;
        "#,
        email.as_ref()
    )
    .fetch_one(conn)
    .await;

    match res {
        Ok(record) => {
            tracing::info!("account found!");
            Ok(record)
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                let message = "account not found!";
                tracing::info!(message);
                Err(message.into())
            }
            _ => {
                tracing::error!("error: {}", e.to_string());
                Err("something went wrong!".into())
            }
        },
    }
}

#[tracing::instrument(name = "saving account to database", skip(conn))]
pub async fn register(conn: &PgPool, input: &RegisterInput) -> Result<Account, Error> {
    let res = sqlx::query_as!(
        Account,
        r#"
            INSERT INTO account (id, name, email)
            VALUES ($1, $2, $3) 
            RETURNING id, name, email, created_at
        "#,
        Uuid::now_v7(),
        input.name,
        input.email.to_string()
    )
    .fetch_one(conn)
    .await;

    match res {
        Ok(record) => {
            tracing::info!("success create account!");
            Ok(record)
        }
        Err(e) => match e {
            sqlx::Error::Database(db_error) => {
                if db_error.is_unique_violation() {
                    let message = "account is already exists!";
                    tracing::info!(message);
                    return Err(message.into());
                }
                tracing::error!("error: {:?}", db_error);
                Err("something went wrong!".into())
            }
            _ => {
                tracing::error!("error: {}", e.to_string());
                Err("something went wrong".into())
            }
        },
    }
}
