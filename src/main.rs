use std::env;

use axum::Router;
mod controllers;
mod models;
mod pool;
mod strategies;
mod middleware;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // load environment variables with dotenv
    let dotenv_path = dotenv::dotenv().expect("failed to find .env file");
    println!("cargo:rerun-if-changed={}", dotenv_path.display());
    let env_path = env::current_dir().and_then(|a| Ok(a.join("/.env"))).unwrap();
    let _ = dotenv::from_path(env_path);

    // Warning: `dotenv_iter()` is deprecated! Roll your own or use a maintained fork such as `dotenvy`.
    // for env_var in dotenv::dotenv_iter().unwrap() {
    //     let (key, value) = env_var.unwrap();
    //     println!("cargo:rustc-env={key}={value}");
    // }

    // create DB Pool
    let _  = pool::create_pool().await;

    // build our application with a route
    let app = Router::new()
        // add cookie manager middleware
        // .layer(CookieManagerLayer::new())
        // nest routers from controllers
        .nest("/auth", controllers::auth_controller::routes())
        .nest("/users", controllers::users_controller::routes());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}