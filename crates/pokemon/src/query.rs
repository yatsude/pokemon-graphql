use async_graphql::{Context, Error, Object};
use sqlx::PgPool;

use crate::{object::Pokemon, repo};

#[derive(Default, Debug)]
pub struct PokemonQuery {}

#[Object]
impl PokemonQuery {
    #[tracing::instrument(name = "get one pokemon by id", skip(ctx))]
    async fn pokemon_by_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: i32,
    ) -> async_graphql::Result<Pokemon, Error> {
        let conn = ctx.data::<PgPool>()?;

        let pokemon = repo::get_pokemon_by_id(conn, id).await?;
        Ok(pokemon)
    }
}
