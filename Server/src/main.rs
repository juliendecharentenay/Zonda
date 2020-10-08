use actix_files;
use actix_files::NamedFile;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, Result};
use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use std::fs;
use serde::{Serialize, Deserialize};

mod simulation;
use simulation::Simulation;
mod mesh;
mod utilities;

fn get_file_path(filename:& String) -> PathBuf {
    env::current_dir().unwrap().join("..").join("store").join(filename)
}

/*
#[post["/api/make_mesh"]]
async fn api_make_mesh(req_body: String) -> impl Responder {
    println!("in api_make_mesh with body {}", req_body);
    let mut s: Simulation = simulation::make_simulation(serde_json::from_str(&req_body).unwrap());
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
*/

#[derive(Serialize, Deserialize)]
struct Response {
  message: String,
}

/*

#[post["/api/get_section"]]
async fn api_get_section(req_body: String) -> impl Responder {
    println!("in api_get_mesh with body {}", req_body);
    let r: GetSectionRequest = serde_json::from_str(&req_body).unwrap();
    let i_file_path = get_file_path(&r.uuid);
    let mut i_file = File::open(i_file_path).unwrap();
    let s: Simulation = simulation::read_simulation(&mut i_file);
    s.print_info();
    HttpResponse::Ok().body("Response from get mesh!")
}
*/



#[post("/api/run_simulation")]
async fn api_run_simulation(req_body: String) -> impl Responder {
    println!("in api_run_simulation with body {}", req_body);
    // let p: Parameters = serde_json::from_str(&req_body).unwrap();
    // println!("Simulation name {} with id {}", p.name, p.project_id);
    // let mut s: simulation::Simulation = simulation::make_simulation(p);
    let mut s: Simulation = simulation::make_simulation(serde_json::from_str(&req_body).unwrap());
    match s.initialize() {
      Err(e) => println!("Error in s.initialize(): {:?}", e), // TODO: Change to return an error message
      _ => ()
    }
    let o_file_path = get_file_path(s.get_uuid());
    let mut o_file = File::create(o_file_path).unwrap();
    s.output(&mut o_file).ok();
    s.print_info();
    // HttpResponse::Ok().json(MyObj { name: parameters.name })
    HttpResponse::Ok().json(Response { message: "Hello".to_string() })
}

/*
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
*/

#[test]
fn run_simulation_test1() {
      println!("Run simulation test 1");
      let mut s: Simulation = simulation::make_simulation_test1();
      match s.initialize() {
        Err(e) => panic!("Error running simulation: {:?}", e),
        _ => (),
      }

      let mut filename = get_file_path(s.get_uuid());
      if Path::new(&filename).exists() { fs::remove_file(filename); }
      filename = get_file_path(s.get_uuid());
      assert!(! Path::new(&filename).exists(), "Simulation file exists before it should be written");
      {
        let mut o_file = File::create(&filename).unwrap();
        s.output(&mut o_file).unwrap();
      }
      assert!(Path::new(&filename).exists(), "Simulation file has not been created");

      filename = get_file_path(&(s.get_uuid().to_owned() + "_parameters.json"));
      if Path::new(&filename).exists() { fs::remove_file(filename); }
      filename = get_file_path(&(s.get_uuid().to_owned() + "_parameters.json"));
      assert!(! Path::new(&filename).exists(), "JSON parameter file exists before it should be written");
      {
        let mut o_file = File::create(&filename).unwrap();
        s.get_parameters().output(&mut o_file);
      }
      assert!(Path::new(&filename).exists(), "JSON parameter file has not been created");

      filename = get_file_path(&(s.get_uuid().to_owned() + ".vtk"));
      if Path::new(&filename).exists() { fs::remove_file(filename); }
      filename = get_file_path(&(s.get_uuid().to_owned() + ".vtk"));
      assert!(! Path::new(&filename).exists(), "VTK file exists before it should be written");
      {
        let mut o_file = File::create(&filename).unwrap();
        utilities::to_legacy_vtk(&s, &mut o_file).unwrap();
      }
      assert!(Path::new(&filename).exists(), "VTK file has not been created");
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
          // .service(api_make_mesh)
          // .service(api_get_section)
          .service(actix_files::Files::new("/", www_directory.as_path())
                       .index_file("index.html"))
          // .route("/", web::get().to(index_html))
          // .service(echo)
          // .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("127.0.0.1:{}",port))?
    .run()
    .await
}


