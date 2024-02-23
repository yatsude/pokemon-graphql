use serde_json::json;
use uuid::Uuid;

use schema::pokemon::Pokemon;

use crate::utils::spawn_graphql;

#[tokio::test]
async fn success_create_pokemon() {
    let schema = spawn_graphql().await.finish();

    let pokemon = Pokemon {
        _id: Uuid::now_v7().into(),
        id: 1,
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
                id: {},
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
        pokemon.id,
        pokemon.name,
        pokemon.base_experience,
        pokemon.height,
        pokemon.is_default,
        pokemon.order
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

    assert_eq!(*data.get("id").unwrap(), json!(pokemon.id));
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
async fn success_find_pokemon_by_gen_id() {
    let schema = spawn_graphql().await.finish();

    let pokemon = Pokemon {
        _id: Uuid::now_v7().into(),
        id: 1,
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
                id: {},
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
        pokemon.id,
        pokemon.name,
        pokemon.base_experience,
        pokemon.height,
        pokemon.is_default,
        pokemon.order
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

    let req = format!(
        r#"
            query {{
              pokemonById(id: {}) {{
                id name baseExperience height isDefault order
              }}
            }}
        "#,
        pokemon.id
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

    assert_eq!(*data.get("id").unwrap(), json!(pokemon.id));
    assert_eq!(*data.get("name").unwrap(), json!(pokemon.name));
    assert_eq!(
        *data.get("baseExperience").unwrap(),
        json!(pokemon.base_experience)
    );
    assert_eq!(*data.get("height").unwrap(), json!(pokemon.height));
    assert_eq!(*data.get("isDefault").unwrap(), json!(pokemon.is_default));
    assert_eq!(*data.get("order").unwrap(), json!(pokemon.order));
}
