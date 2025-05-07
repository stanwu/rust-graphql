// 匯入 Actix-web 和 Juniper 所需的模組
use actix_web::{web, App, HttpServer, Responder};
use actix_web::web::Data; // 用於在處理器之間共享狀態
use juniper::{EmptyMutation, EmptySubscription, RootNode}; // Juniper 的 GraphQL 元件
use juniper_actix::post_graphql_handler; // Actix-web 與 Juniper 的整合
use std::sync::Arc; // 用於線程安全的共享所有權

// 定義 GraphQL 的查詢根
struct Query;

// 為 Query 結構實現 GraphQL 物件
#[juniper::graphql_object]
impl Query {
    // 定義一個 echo 查詢，回傳輸入的訊息
    fn echo(message: String) -> String {
        message
    }
}

// 定義 GraphQL 架構，包含 Query、EmptyMutation 和 EmptySubscription
// EmptyMutation 和 EmptySubscription 在此作為簡化的佔位符
type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

// 定義處理 GraphQL 請求的處理器
// 此函數將 Actix-web 與 Juniper 的 GraphQL 處理器整合
async fn graphql(
    schema: Data<Arc<Schema>>, // 用 Arc 包裝的共享架構，確保線程安全
    req: actix_web::HttpRequest, // 傳入的 HTTP 請求
    payload: web::Payload, // 請求的有效負載（例如 JSON 主體）
) -> impl Responder {
    // 使用 Juniper 的 post_graphql_handler 處理請求
    post_graphql_handler(Arc::as_ref(schema.get_ref()), &(), req, payload).await
}

// 啟動 Actix-web 伺服器的主函數
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 建立 GraphQL 架構
    let schema = Arc::new(Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()));

    // 啟動 Actix-web 伺服器
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone())) // 與所有處理器共享架構
            .route("/graphql", web::post().to(graphql)) // GraphQL 請求的路由
    })
    .bind("127.0.0.1:8080")? // 將伺服器綁定到本地端口 8080
    .run() // 運行伺服器
    .await // 等待伺服器完成
}
