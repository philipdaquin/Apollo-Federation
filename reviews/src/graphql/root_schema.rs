use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
use super::modules::schema::QueryReviews;

#[derive(MergedObject, Default)]
pub struct Query(QueryReviews);

#[derive(MergedObject, Default)]
pub struct Mutation;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, EmptyMutation, EmptySubscription>;
