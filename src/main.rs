use actix_web::{web, App, HttpServer, HttpResponse};
use mongodb::{Client, options::ClientOptions, bson::{doc, Document}};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

async fn get_users() -> HttpResponse {
    dotenv().ok();
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not set");
    let client_options = ClientOptions::parse(&mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("rust_mongo_db");
    let collection = db.collection::<Document>("users");

    let cursor = collection.find(None, None).await.unwrap();
    let users: Vec<User> = cursor
        .map(|doc| {
            let doc = doc.unwrap();
            User {
                name: doc.get_str("name").unwrap().to_string(),
                email: doc.get_str("email").unwrap().to_string(),
            }
        })
        .collect()
        .await;

    HttpResponse::Ok().json(users)
}

async fn create_user(user: web::Json<User>) -> HttpResponse {
    dotenv().ok();
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not set");
    let client_options = ClientOptions::parse(&mongo_url).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("rust_mongo_db");
    let collection = db.collection("users");

    let new_user = doc! {
        "name": &user.name,
        "email": &user.email,
    };

    collection.insert_one(new_user, None).await.unwrap();
    HttpResponse::Created().json(user.0.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
