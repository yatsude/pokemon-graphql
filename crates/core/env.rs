use std::env;

enum Environment {
    Development,
    Production,
    Test,
}

impl From<String> for Environment {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "DEVELOPMENT" => Environment::Development,
            "PRODUCTION" => Environment::Production,
            "TEST" => Environment::Test,
            _ => Environment::Development,
        }
    }
}

impl From<Environment> for String {
    fn from(value: Environment) -> String {
        match value {
            Environment::Production => "PRODUCTION".into(),
            Environment::Development => "DEVELOPMENT".into(),
            Environment::Test => "TEST".into(),
        }
    }
}

pub fn load_env() {
    let environment: Environment = env::var("ENV")
        .unwrap_or(Environment::Development.into())
        .into();

    match environment {
        Environment::Production => {}
        _ => {
            dotenvy::dotenv().ok();
        }
    };
}
