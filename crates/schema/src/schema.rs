use async_graphql::extensions::Tracing;
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

use crate::account::{AccountMutation, AccountQuery};
use crate::pokemon::{PokemonMutation, PokemonQuery};

#[derive(MergedObject, Default)]
pub struct Query(PokemonQuery, AccountQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(PokemonMutation, AccountMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;

pub fn build() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).extension(Tracing)
}
