use async_graphql::{Context, Error, Object};
use sqlx::PgPool;
use uuid::Uuid;

use crate::pokemon::{repo, Pokemon};

#[derive(Default, Debug)]
pub struct PokemonQuery {}

#[Object]
impl PokemonQuery {
    #[tracing::instrument(name = "get one pokemon by id", skip(ctx))]
    async fn pokemon_by_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: Uuid,
    ) -> async_graphql::Result<Pokemon, Error> {
        let conn = ctx.data::<PgPool>()?;

        repo::get_pokemon_by_id(conn, id).await
    }
}
