use std::fmt::Display;

use async_graphql::*;
use serde::Serialize;
use sqlx::Type;

#[derive(Default, Type, Debug, Clone, Serialize)]
pub struct Email(String);

#[Scalar]
impl ScalarType for Email {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            if fast_chemail::is_valid_email(value) {
                Ok(value.parse().map(Email)?)
            } else {
                Err(InputValueError::custom("not an email!"))
            }
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Email(value)
    }
}

impl From<&str> for Email {
    fn from(value: &str) -> Self {
        Email(value.to_string())
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
