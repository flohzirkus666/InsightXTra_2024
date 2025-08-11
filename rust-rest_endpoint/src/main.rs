use actix_cors::Cors;
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

async fn receive_data(data: web::Json<FormData>) -> HttpResponse {
    // receiving request from web-frontend
    create_nfs_export(data.0.volume_name, data.0.size, data.0.prefix)
        .await
        .expect("Failed to create NFS share!");

    HttpResponse::Ok().body("data")
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/nfs_share").route(web::post().to(receive_data)))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await?;

    Ok(())
}
