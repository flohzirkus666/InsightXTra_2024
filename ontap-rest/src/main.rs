use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use anyhow::{Ok, Result};
use env_logger::Env;
use ontap::create_nfs_export;
use serde::{Deserialize, Serialize};

mod env_vars;
mod ontap;

#[derive(Serialize, Deserialize, Debug)]
struct FormData {
    volume_name: String,
    size: u16,
    prefix: String,
}

async fn recieve_data(data: web::Json<FormData>) -> HttpResponse {
    // recieving request from web-frontend
    create_nfs_export(data.0.volume_name, data.0.size, data.0.prefix)
        .await
        .expect("Failed to create NFS share!");

    HttpResponse::Ok().body("data")
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/create_nfs").route(web::post().to(recieve_data)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
