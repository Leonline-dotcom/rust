mod models;

use actix_web::{get, post, web::{self, scope}, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use reqwest;
use futures::StreamExt;
use mongodb::{bson::doc, options, Client, Collection, Cursor, IndexModel};
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use tokio;
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

#[derive(Serialize, Deserialize,Debug)]
struct Review {
    title: String,
    username: String,
    review: String,
    posterpath: String,
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
    let api_url = format!("{}/discover/movie?sort_by=popularity.desc&api_key={}", BASE_URL, api_key);
    match reqwest::get(api_url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => HttpResponse::Ok().body(text),
            Err(_) => HttpResponse::InternalServerError().body("Failed to read API response"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch data from external API"),
    }   
}
#[get("/movies/search/{searchTerm}")]
async fn search_movies(path: web::Path<String>) -> impl Responder {
    let search_term = path.into_inner();
    let api_key = get_api_key().await;
    let api_url = format!("{}/search/movie?api_key={}&query={}", BASE_URL, api_key,search_term);
    println!("{}/search/movie?api_key={}&query={}", BASE_URL, api_key,search_term);
    match reqwest::get(api_url).await {
        Ok(resp) => match resp.text().await {
            Ok(text) => HttpResponse::Ok().body(text),
            Err(_) => HttpResponse::InternalServerError().body("Failed to read API response"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch data from external API"),
    }
}

#[get("/reviews")]
async fn get_reviews(client: web::Data<Client>)-> impl Responder{
    let collection: Collection<Review> =client.database(DB_NAME).collection::<Review>(COLL_NAME);
    println!("you accessed it");
    let mut reviews: Vec<Review> = Vec::new();
    match collection.find(doc! {}).await  {
        Ok(mut cursor) => {
            println!("Cursor retrieved, iterating over reviews");
            while let Some(doc_result) = cursor.next().await {
                match doc_result {
                    Ok(review) => {
                        reviews.push(review);
                    }
                    Err(e) => {
                        println!("Error deserializing document: {:?}", e);
                        return HttpResponse::InternalServerError().body("Error reading a review document");
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to retrieve cursor: {:?}", e); 
            return HttpResponse::InternalServerError().body("Failed to query the database");
        }
    }

    match serde_json::to_string(&reviews) {
        Ok(json) => HttpResponse::Ok().body(json), 
        Err(e) => {
            println!("Failed to serialize reviews: {:?}", e);  
            HttpResponse::InternalServerError().body("Failed to serialize reviews")
        }
    }
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {

dotenv().ok();

let mongodb_uri = get_uri().await;

//     // A Client is needed to connect to MongoDB:
let client = Client::with_uri_str(mongodb_uri).await.expect("failed to connect");

HttpServer::new(move || {
    App::new()
        .app_data(web::Data::new(client.clone()))  // Share MongoDB client across requests
        .service(submit_review)  // Route to submit reviews
        .service(get_movies)
        .service(search_movies)
        .service(get_reviews)
        .service(fs::Files::new("/", "./frontend").index_file("index.html"))
})
.bind(("127.0.0.1", 8080))?
.run()
.await
}