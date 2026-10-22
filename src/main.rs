use crate::state::AppState;

mod handlers;
mod models;
mod routes;
mod state;
#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = routes::app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    println!("The service is running at http://localhost:3001");

    axum::serve(listener, app).await.unwrap();
}
