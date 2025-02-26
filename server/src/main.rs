use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use futures::executor::block_on;
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer, AllowOrigin};
use std::env;
use std::sync::Arc;
use axum::{routing::get, Json, Router};
use axum::extract::State;
use dotenvy::dotenv;
use serde::Serialize;
mod entities;

async fn run() -> anyhow::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    let db = Database::connect(database_url).await?;
    let db = Arc::new(db);

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(AllowOrigin::any())
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(get_foo))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(db.clone());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(error) = block_on(run()) {
        panic!("{:?}", error);
    }
}

#[derive(Serialize)]
struct PetResponse {
    id: i32,
    name: String,
    size: String,
    birthday: String,
    weight: f64,
}


async fn get_foo(State(db): State<Arc<DatabaseConnection>>) -> Json<Vec<PetResponse>> {
    use entities::pet::Entity as Pet;

    let pets = Pet::find()
        .all(&*db)
        .await
        .expect("Failed to fetch pets");

    let response: Vec<PetResponse> = pets
        .into_iter()
        .map(|pet| PetResponse {
            id: pet.pet_id,
            name: pet.name,
            size: pet.size.unwrap_or_else(|| "N/A".to_string()),
            birthday: pet.birthday.map(|b| b.to_string()).unwrap_or_else(|| "N/A".to_string()),
            weight: pet.weight.unwrap_or_else(|| -1.0),
        })
        .collect();

    Json(response)
}