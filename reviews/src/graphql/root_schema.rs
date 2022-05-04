use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
use super::modules::schema::{QueryReviews, MutationReviews};

#[derive(MergedObject, Default)]
pub struct Query(QueryReviews);

#[derive(MergedObject, Default)]
pub struct Mutation(MutationReviews);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
