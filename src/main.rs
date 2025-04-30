use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::error::HttpError;
use clap::Parser;
use env_logger::Env;
use log::{info};
use ross::local_storage;

#[get("")]
async fn get_stores() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("")]
async fn create_store(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("foo")
}

#[get("/{store_id}")]
async fn describe_store(path: web::Path<String>) -> Result<String, HttpError> {
    let store_id = path.into_inner();
    Ok(format!("Welcome store_id {}!", store_id))
}

#[delete("/{store_id}")]
async fn delete_store(path: web::Path<String>) -> Result<String, HttpError> {
    let store_id = path.into_inner();
    Ok(format!("Welcome store_id {}!", store_id))
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RossServerCli {
    root_dir: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init the logger
    let env = Env::default()
        .default_filter_or("info");
    env_logger::init_from_env(env);

    let cli = RossServerCli::parse();
    info!("Using {0} as the root directory.", cli.root_dir);

    let storage = local_storage::load(&cli.root_dir)?;
    info!("Storage: {:?}", storage);

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/stores")
                     .service(get_stores)
                     .service(create_store)
                     .service(describe_store)
                     .service(delete_store))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
