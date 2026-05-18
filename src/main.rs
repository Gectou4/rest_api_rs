use tower_http::cors::CorsLayer;

use g4api::config::db::create_pool;
use g4api::routes::create_router;
use g4api::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:root@localhost:3306/rest_api".to_string());

    println!("Connecting to database...");
    let pool = loop {
        match create_pool(&database_url).await {
            Ok(p) => break p,
            Err(e) => {
                eprintln!("Database connection failed: {e}, retrying in 2s...");
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    };

    let state = AppState { pool };

    let app = create_router(state).layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
