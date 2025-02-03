use actix_web::{web, App, HttpServer, Responder, HttpResponse, post};
use actix_web::rt::task;
use serde::{Deserialize, Serialize};
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType, Embedding};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize)]
struct LogRequest {
    log: String,
}

struct AppState {
    model: Arc<Mutex<rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel>>,
}

#[post("/get_log_embedding")]
async fn get_log_embedding(
    req: web::Json<LogRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    println!("Embedding the log line: {}", req.log);

    let log_text = req.log.clone();
    let model = data.model.lock().unwrap(); // Access the shared model

    let embeddings = model
        .encode(&[log_text])
        .expect("Failed to encode text");

    HttpResponse::Ok().json(embeddings[0].clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8089");

    // Load the model once when starting the server
    let model = task::spawn_blocking(move || {
        SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
            .create_model()
            .expect("Failed to load embedding model")
    })
        .await?;

    // Sharing this model across threads was a pain, and just created mutex for the same reason
    // I am not sure why this model gets mutated
    // Creating the model per request is never a choice as it takes 5-10 seconds
    let app_state = web::Data::new(AppState {
        model: Arc::new(Mutex::new(model)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Share the state
            .service(get_log_embedding)
    })
        .bind("127.0.0.1:8089")?
        .run()
        .await
}
