use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    async fn dictionary(&self) -> crate::result::Result<&str> {
        todo!()
    }
}
