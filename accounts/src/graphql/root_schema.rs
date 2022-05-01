use async_graphql::{EmptySubscription, 
    MergedObject, Schema, SchemaBuilder, EmptyMutation};
use super::modules::schema::{UserQuery, UserMutation};


#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
pub type AppSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;
