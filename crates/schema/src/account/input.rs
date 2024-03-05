use async_graphql::*;
use serde::Serialize;

use shared::scalars::Email;
use shared::validate::Validate;

#[derive(InputObject, Debug, Serialize)]
pub struct RegisterInput {
    pub email: Email,

    #[graphql(validator(min_length = 5, max_length = 256))]
    pub name: String,
}

impl Validate for RegisterInput {
    fn validate(&self) -> Result<(), Error> {
        if self.name.trim().len() < 5 {
            Err("name is too short!".into())
        } else {
            Ok(())
        }
    }
}
