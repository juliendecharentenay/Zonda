use serde::{Serialize, Deserialize};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct BoundaryParameters {
  pub uuid: String,
  pub name: String,
  pub bctype: String,
  pub geometry: Option<String>,
  pub vx: f64,
  pub vy: f64,
  pub vz: f64,
  pub p: f64,
  pub t: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Parameters {
  pub uuid: String,
  pub name: String,
  pub project_id: String,
  pub x_center: f64,
  pub y_center: f64,
  pub z_center: f64,
  pub x_n_points: usize,
  pub y_n_points: usize,
  pub z_n_points: usize,
  pub mesh_size: f64,
  pub time_step: f64,
  pub n_time_steps: usize,
  pub norm_velocity: f64,
  pub norm_density: f64,
  pub norm_enthalpy: f64,
  pub boundaries: Vec<BoundaryParameters>,
}

impl Parameters {
  /**
   * Output parameters to object implementing Write trait
   */
  pub fn output(&self, o:&mut std::fs::File) {
    o.write_all(serde_json::to_string(&self).unwrap().as_bytes()).unwrap();
  }
}
