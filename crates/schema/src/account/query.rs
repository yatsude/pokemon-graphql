use async_graphql::*;
use sqlx::PgPool;

use shared::scalars::Email;

use crate::account::{repo, Account};

#[derive(Default, Debug)]
pub struct AccountQuery {}

#[Object]
impl AccountQuery {
    #[tracing::instrument(name = "get one account by email", skip(ctx))]
    async fn account_by_email<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        email: Email,
    ) -> Result<Account, Error> {
        let conn = ctx.data::<PgPool>()?;

        repo::get_account_by_email(conn, email).await
    }
}
