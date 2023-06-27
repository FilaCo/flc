use async_trait::async_trait;

#[async_trait]
pub trait Repo {
    type Item;
    type CreateQueryCriteria;
    type ReadQueryCriteria;
    type UpdateQueryCriteria;
    type DeleteQueryCriteria;

    async fn create(&self, query_criteria: &Self::CreateQueryCriteria);
    async fn read(&self, query_criteria: &Self::ReadQueryCriteria);
    async fn update(&self, query_criteria: &Self::UpdateQueryCriteria);
    async fn delete(&self, query_criteria: &Self::DeleteQueryCriteria);
}
