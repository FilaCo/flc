use api::::Repo;
use async_trait::async_trait;

pub struct Translation {}

#[async_trait]
impl Repo for Translation {
    type Item = ();
    type CreateQueryCriteria = ();
    type ReadQueryCriteria = ();
    type UpdateQueryCriteria = ();
    type DeleteQueryCriteria = ();

    async fn create(&self, query_criteria: &Self::CreateQueryCriteria) {
        todo!()
    }

    async fn read(&self, query_criteria: &Self::ReadQueryCriteria) {
        todo!()
    }

    async fn update(&self, query_criteria: &Self::UpdateQueryCriteria) {
        todo!()
    }

    async fn delete(&self, query_criteria: &Self::DeleteQueryCriteria) {
        todo!()
    }
}
