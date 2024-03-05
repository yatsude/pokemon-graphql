use async_graphql::Response;
use serde_json::json;

use schema::account::RegisterInput;
use schema::AppSchema;

use crate::utils::spawn_graphql;

#[tokio::test]
async fn register_bad_request() {
    let schema = spawn_graphql().await.finish();

    let test_cases = vec![
        ("yatsude@", "yatsude", "bad email"),
        ("yatsude.ne@gmail.com", "y", "short name"),
        ("", "yatsude", "empty email"),
        ("yatsude.ne@gmail.com", "", "empty name"),
        ("yatsude.ne@gmail.com", "     a", "invalid name with space"),
    ];

    for tc in test_cases {
        let res = schema
            .execute(format!(
                r#"
                    mutation {{
                        register(input: {{
                            email: "{}",
                            name: "{}"
                        }})
                        {{ name email }}
                    }}
                "#,
                tc.0, tc.1
            ))
            .await;
        assert!(res.is_err());
    }
}

#[tokio::test]
async fn registered_email() {
    let schema = spawn_graphql().await.finish();

    async fn do_request(schema: &AppSchema, email: String) -> Response {
        schema
            .execute(format!(
                r#"
                    mutation {{
                        register(input: {{
                            email: "{}",
                            name: "badri"
                        }})
                        {{ name email }}
                    }}
                "#,
                email
            ))
            .await
    }

    let email = "badri.ula@gmail.com";

    let res = do_request(&schema, email.to_string()).await;
    assert!(res.is_ok());
    let data = res
        .data
        .into_json()
        .unwrap()
        .get("register")
        .unwrap()
        .clone();
    assert_eq!(*data.get("email").unwrap(), json!(email));

    let res = do_request(&schema, email.into()).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn register_success() {
    let schema = spawn_graphql().await.finish();

    let input = RegisterInput {
        name: "yatsude".into(),
        email: "yatsude.ne@gmail.com".into(),
    };

    let res = schema
        .execute(format!(
            r#"
                mutation {{
                    register(input: {{
                        email: "{}",
                        name: "{}"
                    }})
                    {{ name email }}
                }}
            "#,
            input.email, input.name
        ))
        .await;

    assert!(res.is_ok());
    let data = res
        .data
        .into_json()
        .unwrap()
        .get("register")
        .unwrap()
        .clone();

    assert_eq!(data, json!(input))
}

#[tokio::test]
async fn find_account_by_email_success() {
    let schema = spawn_graphql().await.finish();

    let input = RegisterInput {
        name: "yatsude".into(),
        email: "yatsude.ne@gmail.com".into(),
    };

    let res = schema
        .execute(format!(
            r#"
                mutation {{
                    register(input: {{
                        email: "{}",
                        name: "{}"
                    }})
                    {{ name email }}
                }}
            "#,
            input.email, input.name
        ))
        .await;
    assert!(res.is_ok());

    let res = schema
        .execute(format!(
            r#"
                query {{
                    accountByEmail(email: "{}")
                    {{ id name email createdAt }}
                }}
            "#,
            input.email
        ))
        .await;
    assert!(res.is_ok());
    let data = res
        .data
        .into_json()
        .unwrap()
        .get("accountByEmail")
        .unwrap()
        .clone();
    assert!(data.get("id").is_some());
    assert!(data.get("createdAt").is_some());
    assert_eq!(*data.get("name").unwrap(), json!(input.name));
    assert_eq!(*data.get("email").unwrap(), json!(input.email));
}
