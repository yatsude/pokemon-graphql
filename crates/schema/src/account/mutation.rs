use async_graphql::{Context, Error, Object};
use sqlx::PgPool;

use shared::validate::Validate;

use crate::account::{repo, Account, RegisterInput};

#[derive(Default, Debug)]
pub struct AccountMutation {}

#[Object]
impl AccountMutation {
    #[tracing::instrument(name = "register an account", skip(ctx))]
    async fn register<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        input: RegisterInput,
    ) -> async_graphql::Result<Account, Error> {
        let conn = ctx.data::<PgPool>()?;
        input.validate()?;

        repo::register(conn, &input).await
    }
}
