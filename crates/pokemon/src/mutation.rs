use async_graphql::{Context, Error, Object};
use sqlx::PgPool;

use crate::{input::CreatePokemonInput, object::Pokemon, repo};

#[derive(Default, Debug)]
pub struct PokemonMutation {}

#[Object]
impl PokemonMutation {
    #[tracing::instrument(name = "adding new pokemon", skip(ctx))]
    async fn create_pokemon<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: CreatePokemonInput,
    ) -> async_graphql::Result<Pokemon, Error> {
        let conn = ctx.data::<PgPool>()?;
        let pokemon = repo::create_pokemon(conn, &input).await?;
        Ok(pokemon)
    }
}
