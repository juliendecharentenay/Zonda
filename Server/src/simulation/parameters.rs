use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Parameters {
  pub uuid: String,
  pub name: String,
  pub project_id: String,
  pub x_center: f64,
  pub x_n_points: usize,
  pub y_center: f64,
  pub y_n_points: usize,
  pub z_center: f64,
  pub z_n_points: usize,
  pub mesh_size: f64,
  pub time_step: f64,
  pub n_time_steps: usize,
}

