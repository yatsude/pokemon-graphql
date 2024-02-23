use async_graphql::Error;
use sqlx::PgPool;
use uuid::Uuid;

use crate::pokemon::{CreatePokemonInput, Pokemon};

#[tracing::instrument(name = "get one pokemon from database", skip(conn))]
pub async fn get_pokemon_by_id(conn: &PgPool, id: i32) -> async_graphql::Result<Pokemon, Error> {
    let res = sqlx::query_as!(
        Pokemon,
        r#"
            SELECT _id, id, name, base_experience, height, is_default, "order" FROM pokemon WHERE id = $1
        "#,
        id,
    )
        .fetch_one(conn)
        .await;

    match res {
        Ok(record) => {
            tracing::info!("pokemon found!");
            Ok(record)
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                let message = "pokemon not found!";
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

#[tracing::instrument(name = "create pokemon to database", skip(conn))]
pub async fn create_pokemon(
    conn: &PgPool,
    input: &CreatePokemonInput,
) -> async_graphql::Result<Pokemon, Error> {
    let res = sqlx::query_as!(
        Pokemon,
        r#"
            INSERT INTO pokemon (_id, id, name, base_experience, height, is_default, "order")
            VALUES ($1, $2, $3, $4, $5, $6, $7) 
            RETURNING _id, id, name, base_experience, height, is_default, "order"
        "#,
        Uuid::now_v7(),
        input.id,
        input.name,
        input.base_experience,
        input.height,
        input.is_default,
        input.order,
    )
    .fetch_one(conn)
    .await;

    match res {
        Ok(record) => {
            tracing::info!("success create pokemon!");
            Ok(record)
        }
        Err(e) => match e {
            sqlx::Error::Database(db_error) => {
                if db_error.is_unique_violation() {
                    let message = "pokemon is already exists!";
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
