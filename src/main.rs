use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Item {
    id: u32,
    name: String,
    description: String,
}

#[derive(Serialize)]
struct HomeResponse {
    message: String,
    version: String,
    endpoints: Endpoints,
}

#[derive(Serialize)]
struct Endpoints {
    health: String,
    items: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[derive(Serialize)]
struct ItemsResponse {
    items: Vec<Item>,
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().json(HomeResponse {
        message: "Welcome to the Rust API".to_string(),
        version: "1.0.0".to_string(),
        endpoints: Endpoints {
            health: "/health".to_string(),
            items: "/api/items".to_string(),
        },
    })
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
    })
}

#[get("/api/items")]
async fn get_items() -> impl Responder {
    let items = vec![
        Item {
            id: 1,
            name: "Item One".to_string(),
            description: "First example item".to_string(),
        },
        Item {
            id: 2,
            name: "Item Two".to_string(),
            description: "Second example item".to_string(),
        },
        Item {
            id: 3,
            name: "Item Three".to_string(),
            description: "Third example item".to_string(),
        },
    ];
    HttpResponse::Ok().json(ItemsResponse { items })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Server running on port {}", port);

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(health)
            .service(get_items)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
