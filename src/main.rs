use axum::{routing::get, Extension, Router};
use axum_login::{
    tower_sessions::{MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use dotenv::dotenv;

use heyo::routes::{self, admin_routes, protected_routes, public_routes, studio_owner_routes};
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};
use tower::make::Shared;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("Migrations run successfully!");
    // Define app routes
    //let app = app_routes().layer(Extension(shared_pool));
    // Set up the main router and include sub-routers
    let shared_pool = Arc::new(pool);
    let app = Router::new()
        .merge(public_routes::public_routes()) // Merge user-related routes
        .merge(studio_owner_routes::studio_owner_routes()) // Merge studio owner-related routes
        .merge(admin_routes::admin_routes()) // Merge admin-related routes
        .merge(protected_routes::protected_routes(shared_pool.clone())) // Merge admin-related routes
        .route("/", get(root_handler)) // Main root route
        .layer(Extension(shared_pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // Run the server
    axum::serve(listener, app).await.unwrap();
}
/* async fn root() -> &'static str {
    "Hello, world!"
}
 */
async fn root_handler() -> &'static str {
    "Welcome to the API"
}
