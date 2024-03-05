use async_graphql::{Error, Result};

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}
