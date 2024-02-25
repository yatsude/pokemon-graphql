use async_graphql::InputObject;

#[derive(InputObject, Debug)]
pub struct CreatePokemonInput {
    pub name: String,
    pub base_experience: i32,
    pub height: i32,
    pub is_default: bool,
    pub order: i32,
}
