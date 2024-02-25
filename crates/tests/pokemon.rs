use serde_json::json;
use uuid::Uuid;

use schema::pokemon::Pokemon;

use crate::utils::spawn_graphql;

#[tokio::test]
async fn success_create_pokemon() {
    let schema = spawn_graphql().await.finish();

    let pokemon = Pokemon {
        id: Uuid::now_v7(),
        name: "pikachu".into(),
        base_experience: 120,
        height: 4,
        is_default: true,
        order: 1,
    };
    let req = format!(
        r#"
            mutation {{
              createPokemon(input: {{
                name: "{}",
                baseExperience: {},
                height: {},
                isDefault: {},
                order: {}
              }}) {{
                id name baseExperience height isDefault order
              }}
            }}
        "#,
        pokemon.name, pokemon.base_experience, pokemon.height, pokemon.is_default, pokemon.order
    );

    let res = schema.execute(req).await;
    assert!(res.is_ok());

    let data = res
        .data
        .into_json()
        .unwrap()
        .get("createPokemon")
        .unwrap()
        .clone();

    assert!(data.get("id").is_some());
    assert_eq!(*data.get("name").unwrap(), json!(pokemon.name));
    assert_eq!(
        *data.get("baseExperience").unwrap(),
        json!(pokemon.base_experience)
    );
    assert_eq!(*data.get("height").unwrap(), json!(pokemon.height));
    assert_eq!(*data.get("isDefault").unwrap(), json!(pokemon.is_default));
    assert_eq!(*data.get("order").unwrap(), json!(pokemon.order));
}

#[tokio::test]
async fn success_find_pokemon_by_id() {
    let schema = spawn_graphql().await.finish();

    let pokemon = Pokemon {
        id: Uuid::now_v7(),
        name: "pikachu".into(),
        base_experience: 120,
        height: 4,
        is_default: true,
        order: 1,
    };
    let req = format!(
        r#"
            mutation {{
              createPokemon(input: {{
                name: "{}",
                baseExperience: {},
                height: {},
                isDefault: {},
                order: {}
              }}) {{
                id
              }}
            }}
        "#,
        pokemon.name, pokemon.base_experience, pokemon.height, pokemon.is_default, pokemon.order
    );

    let res = schema.execute(req).await;
    assert!(res.is_ok());

    let data = res
        .data
        .into_json()
        .unwrap()
        .get("createPokemon")
        .unwrap()
        .clone();
    assert!(data.get("id").is_some());

    let id = data.get("id").unwrap();

    let req = format!(
        r#"
            query {{
              pokemonById(id: {}) {{
                id name baseExperience height isDefault order
              }}
            }}
        "#,
        id
    );

    let res = schema.execute(req).await;
    assert!(res.is_ok());
    let data = res
        .data
        .into_json()
        .unwrap()
        .get("pokemonById")
        .unwrap()
        .clone();

    assert_eq!(*data.get("id").unwrap(), json!(id));
    assert_eq!(*data.get("name").unwrap(), json!(pokemon.name));
    assert_eq!(
        *data.get("baseExperience").unwrap(),
        json!(pokemon.base_experience)
    );
    assert_eq!(*data.get("height").unwrap(), json!(pokemon.height));
    assert_eq!(*data.get("isDefault").unwrap(), json!(pokemon.is_default));
    assert_eq!(*data.get("order").unwrap(), json!(pokemon.order));
}
