use axum::Router;
use tower_cookies::CookieManagerLayer;
mod controllers;
mod models;
mod pool;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // load environment variables with dotenv
    let dotenv_path = dotenv::dotenv().expect("failed to find .env file");
    println!("cargo:rerun-if-changed={}", dotenv_path.display());

    // Warning: `dotenv_iter()` is deprecated! Roll your own or use a maintained fork such as `dotenvy`.
    for env_var in dotenv::dotenv_iter().unwrap() {
        let (key, value) = env_var.unwrap();
        println!("cargo:rustc-env={key}={value}");
    }

    // create DB Pool
    let _  = pool::create_pool().await;

    // build our application with a route
    let app = Router::new()
        // add cookie manager middleware
        .layer(CookieManagerLayer::new())
        // nest routers from controllers
        .nest("/users", controllers::users_controller::user_routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}