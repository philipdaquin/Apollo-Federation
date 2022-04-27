use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
    use super::modules::schema::QueryProducts;


#[derive(MergedObject, Default)]
pub struct Query(QueryProducts);

#[derive(MergedObject, Default)]
pub struct Mutation;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, EmptyMutation, EmptySubscription>;
