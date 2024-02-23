use async_graphql::Object;

#[derive(Default)]
pub struct Pokemon {
    pub _id: String,
    pub id: i32,
    pub name: String,
    pub base_experience: i32,
    pub height: i32,
    pub is_default: bool,
    pub order: i32,
}

#[Object]
impl Pokemon {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        self.name.as_str()
    }

    async fn base_experience(&self) -> i32 {
        self.base_experience
    }

    async fn height(&self) -> i32 {
        self.height
    }

    async fn is_default(&self) -> bool {
        self.is_default
    }

    async fn order(&self) -> i32 {
        self.order
    }
}
