use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, Result};
use std::path::{PathBuf};
use std::env;
use std::fs::File;
use serde::{Serialize, Deserialize};

mod simulation;

// #[path = "simulation/simulation.rs"] mod simulation;
// #[path = "simulation/parameters.rs"] mod parameters;


fn get_file_path(filename:& String) -> PathBuf {
    env::current_dir().unwrap().join("..").join("store").join(filename)
}

#[post["/api/make_mesh"]]
async fn api_make_mesh(req_body: String) -> impl Responder {
    println!("in api_make_mesh with body {}", req_body);
    let mut s: simulation::Simulation = simulation::make_simulation(serde_json::from_str(&req_body).unwrap());
    s.initialize();
    let o_file_path = get_file_path(s.get_uuid());
    let mut o_file = File::create(o_file_path).unwrap();
    s.output(&mut o_file).unwrap();
    s.print_info();
    HttpResponse::Ok().body("Response from make mesh!")
}

#[derive(Serialize, Deserialize)]
struct GetSectionRequest {
  uuid: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
  message: String,
}

#[post["/api/get_section"]]
async fn api_get_section(req_body: String) -> impl Responder {
    println!("in api_get_mesh with body {}", req_body);
    let r: GetSectionRequest = serde_json::from_str(&req_body).unwrap();
    let i_file_path = get_file_path(&r.uuid);
    let mut i_file = File::open(i_file_path).unwrap();
    let s: simulation::Simulation = simulation::read_simulation(&mut i_file);
    s.print_info();
    HttpResponse::Ok().body("Response from get mesh!")
}

#[post("/api/run_simulation")]
async fn api_run_simulation(req_body: String) -> impl Responder {
    println!("in api_run_simulation with body {}", req_body);
    // let p: Parameters = serde_json::from_str(&req_body).unwrap();
    // println!("Simulation name {} with id {}", p.name, p.project_id);
    // let mut s: simulation::Simulation = simulation::make_simulation(p);
    let mut s: simulation::Simulation = simulation::make_simulation(serde_json::from_str(&req_body).unwrap());
    s.initialize();
    let o_file_path = get_file_path(s.get_uuid());
    let mut o_file = File::create(o_file_path).unwrap();
    s.output(&mut o_file).ok();
    s.print_info();
    // HttpResponse::Ok().json(MyObj { name: parameters.name })
    HttpResponse::Ok().json(Response { message: "Hello".to_string() })
}

#[get("/hello")]
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

#[test]
fn run_default_simulation() {
      println!("Run default simulation");
      let mut s: simulation::Simulation = simulation::make_default_simulation();
      s.initialize();
      {
        let mut o_file = File::create(get_file_path(s.get_uuid())).unwrap();
        s.output(&mut o_file).unwrap();
      }
      {
        let mut o_file = File::create(get_file_path(&(s.get_uuid().to_owned() + "_parameters.json"))).unwrap();
        s.get_parameters().output(&mut o_file);
      }
      {
        let mut o_file = File::create(get_file_path(&(s.get_uuid().to_owned() + ".vtk"))).unwrap();
        s.to_legacy_vtk(&mut o_file).unwrap();
      }
      s.print_info();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // println!("{}", fs::Files::new("/www", ".").show_files_listing());
    let port: i32 = 8080;
    let www_directory = env::current_dir().unwrap().join("..").join("www");
    println!("Starting http server on port {} serving files at {}.", port, www_directory.to_str().unwrap());
    HttpServer::new(move || {
      App::new()
          .service(api_run_simulation)
          .service(api_make_mesh)
          .service(api_get_section)
          // .service(hello)
          .service(fs::Files::new("/", www_directory.as_path()) // .show_files_listing()
                       .index_file("index.html"))
          // .route("/", web::get().to(index_html))
          // .service(echo)
          // .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("127.0.0.1:{}",port))?
    .run()
    .await
}


