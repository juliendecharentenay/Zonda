use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use std::path::PathBuf;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn index_html() -> Result<NamedFile> {
    let path = PathBuf::from(r".\www\index.html");
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // println!("{}", fs::Files::new("/www", ".").show_files_listing());
    let port = 8080i32;
    println!("Starting http server on port {}", port);
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./www") // .show_files_listing()
                         .index_file("index.html"))
            // .route("/", web::get().to(index_html))
            // .service(hello)
            // .service(echo)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("127.0.0.1:{}",port))?
    .run()
    .await
}


