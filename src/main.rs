#[macro_use]
extern crate diesel;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[post("/api/addpos")]
async fn add_pos(pool: web::Data<DbPool>, form: web::Json<models::MapPos>) -> Result<HttpResponse, Error>{

    let conn = pool.get().expect("couldn't get db connection from pool");

    web::block(move || actions::insert_new_pos(&form, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/api/pos")]
async fn get_pos(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let conn = pool.get().expect("couldn't get db connection from pool");

    let pos = web::block(move || actions::get_all_pos(&conn)).await.map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    if pos.len() == 0 {
        Ok(HttpResponse::NotFound().finish())
    }else{
        Ok(HttpResponse::Ok().json(pos))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    let bind = "0.0.0.0:8100";

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(add_pos)
            .service(get_pos)
    })
        .bind(&bind)?
        .run()
        .await
}