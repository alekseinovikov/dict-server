// web-server/src/lib.rs
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    dict: dictionary::Dictionary,
}

#[get("/add/{key}/{value}")]
async fn add_element(data: web::Data<AppState>, kv:web::Path<(String, String)>) -> impl Responder {
    let (key, value) = kv.into_inner();
    data.dict.put(key, value).await;
    HttpResponse::Ok().body("Element added")
}

#[get("/add/{key}")]
async fn get_element(data: web::Data<AppState>, key: web::Path<String>) -> impl Responder {
    if let Some(value) = data.dict.get(&key).await {
        HttpResponse::Ok().body(value)
    } else {
        HttpResponse::NotFound().body("Key not found")
    }
}

#[get("/get_all")]
async fn get_all_elements(data: web::Data<AppState>) -> impl Responder {
    let all_elements = data.dict.get_all_as_json().await;
    HttpResponse::Ok().content_type(mime::APPLICATION_JSON).body(all_elements)
}

#[actix_web::main]
pub async fn run_server() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        dict: dictionary::new(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(add_element)
            .service(get_element)
            .service(get_all_elements)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}