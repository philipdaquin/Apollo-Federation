const { ApolloServer } = require('apollo-server');
const { ApolloGateway, RemoteGraphQLDataSource, 
    IntrospectAndCompose } = require('@apollo/gateway');


// Each incoming request to the gateway includes 'Authorization' header
// The gateway sets the shared 'context' for an operation by pulling the value of that 
// header and using it to fetch the associated user's id 
class AuthenticatedDataSource extends RemoteGraphQLDataSource {
    willSendRequest({ request, context }) { 
        // Pass the user's id from the context to each subgraph 
        //  as a header called 'user_id'
        if (context.authHeaderValue) { 
            request.http.headers.set('Authorization', context.authHeaderValue);
        }
    }
}
//  Intialise an ApolloGateway instance and pass it 
//  the supergraph schema 
const gateway = new ApolloGateway({
    supergraphSdl: new IntrospectAndCompose({
        subgraphs: [
            { name: 'accounts', url: 'http://localhost:4001 '},
            { name: 'products', url: 'http://localhost:8082 '},
            { name: 'reviews', url: 'http://localhost:8081 '},
            { name: 'inventory', url: 'http://localhost:8081 '},
        ],
        buildService({name, url}) {
            return new AuthenticatedDataSource({url});
        },
        // Experimental: Enabling this enables the query plan view in Playground.
        __exposeQueryPlanExperimental: false,
    })
});

//  We initialise an 'ApolloServer' instance and pass it our gateway via the gateway optiomn
//  Pass the ApolloGateway to the ApolloServer constructor 
const server = new ApolloServer({
    gateway,
    engine: false,
    mocks: false,
    subscriptions: false,
});

//  We call listen on our server instance to begin listening for incoming request 
server.listen().then(() => {
    console.log(`
      🚀  Server is running!
      🔉  Listening on port 4000
      📭  Query at https://studio.apollographql.com/dev
    `);
  });