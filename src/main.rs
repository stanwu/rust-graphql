use actix_web::{web, App, HttpServer, Responder};
use actix_web::web::Data;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use juniper_actix::post_graphql_handler;
use std::sync::Arc;

struct Query;

#[juniper::graphql_object]
impl Query {
    fn echo(message: String) -> String {
        message
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

async fn graphql(
    schema: Data<Arc<Schema>>,
    req: actix_web::HttpRequest,
    payload: web::Payload,
) -> impl Responder {
    post_graphql_handler(Arc::as_ref(schema.get_ref()), &(), req, payload).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Arc::new(Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
