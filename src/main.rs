// Import necessary modules from Actix-web and Juniper
use actix_web::{web, App, HttpServer, Responder};
use actix_web::web::Data; // For sharing state between handlers
use juniper::{EmptyMutation, EmptySubscription, RootNode}; // Juniper GraphQL components
use juniper_actix::post_graphql_handler; // Actix-web integration for Juniper
use std::sync::Arc; // For thread-safe shared ownership of the schema

// Define the GraphQL query root
struct Query;

// Implement the GraphQL object for the Query struct
#[juniper::graphql_object]
impl Query {
    // Define an echo query that returns the input message
    fn echo(message: String) -> String {
        message
    }
}

// Define the GraphQL schema with Query, EmptyMutation, and EmptySubscription
// EmptyMutation and EmptySubscription are used here as placeholders for simplicity
type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

// Define the handler for GraphQL requests
// This function integrates Actix-web with Juniper's GraphQL handler
async fn graphql(
    schema: Data<Arc<Schema>>, // Shared schema wrapped in Arc for thread safety
    req: actix_web::HttpRequest, // Incoming HTTP request
    payload: web::Payload, // Request payload (e.g., JSON body)
) -> impl Responder {
    // Use Juniper's post_graphql_handler to process the request
    post_graphql_handler(Arc::as_ref(schema.get_ref()), &(), req, payload).await
}

// Main function to start the Actix-web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create the GraphQL schema
    let schema = Arc::new(Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()));

    // Start the Actix-web server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone())) // Share the schema with all handlers
            .route("/graphql", web::post().to(graphql)) // Route for GraphQL requests
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost on port 8080
    .run() // Run the server
    .await // Await the server's completion
}
