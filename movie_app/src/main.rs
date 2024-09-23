mod models;


use actix_web::{get, post, web::{self, scope}, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use reqwest;
use actix_cors::Cors;
use mongodb::{bson::doc, options::IndexOptions, Client,options::{ClientOptions, ResolverConfig}, Collection, IndexModel};
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use std::{env,os,process};
use std::error::Error;


const DB_NAME: &str = "movie_application";
const COLL_NAME: &str = "movie_reviews";
const BASE_URL: &str ="https://api.themoviedb.org/3";

// Getting the url and uri
async fn get_api_key() ->  String{
    env::var("TMB_API_KEY").expect("TMDB_API_KEY must be set").to_string()
}

async fn get_uri() -> String{
    env::var("MONGODB_URI").expect("MONGODB_URI must be set").to_string()
}

#[derive(Serialize, Deserialize)]
struct Review {
    username: String,
    review: String,
}

#[post("/submit_review")]
async fn submit_review(client: web::Data<Client>, review: web::Json<Review>) -> impl Responder {
    let collection: Collection<Review> = client.database(DB_NAME).collection(COLL_NAME);

    match collection.insert_one(review.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Review submitted successfully"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to submit review: {}", err)),
    }
}
#[get("/movies")]
async fn get_movies() -> impl Responder {
    let api_key = get_api_key().await;
    let api_url = format!("{}/discover/movie?sort_by=popularity.desc&{}", BASE_URL, api_key);
    println!("{}",api_url);
    match reqwest::get(api_url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => HttpResponse::Ok().body(text),
            Err(_) => HttpResponse::InternalServerError().body("Failed to read API response"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch data from external API"),
    }
}
#[post("/fetch_external_api")]
async fn fetch_external_api() -> impl Responder {
    let api_url = "https://jsonplaceholder.typicode.com/todos";
    match reqwest::get(api_url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => HttpResponse::Ok().body(text),
            Err(_) => HttpResponse::InternalServerError().body("Failed to read API response"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch data from external API"),
    }
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {

dotenv().ok();

let mongodb_uri = get_uri().await;

//     // A Client is needed to connect to MongoDB:
let client = Client::with_uri_str(mongodb_uri).await.expect("failed to connect");
// //    // Print the databases in our MongoDB cluster:
// //    println!("Databases:");
// //    for name in client.list_database_names().await? {
// //       println!("- {}", name);
// //    }
HttpServer::new(move || {
    App::new()
        .app_data(web::Data::new(client.clone()))  // Share MongoDB client across requests
        .service(submit_review)  // Route to submit reviews
        .service(get_movies)
        .service(fetch_external_api)  // Route to fetch external API
        .service(fs::Files::new("/", "./frontend").index_file("index.html"))
})
.bind(("127.0.0.1", 8080))?
.run()
.await
}