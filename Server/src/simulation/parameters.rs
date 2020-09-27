use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Parameters {
  pub uuid: String,
  pub name: String,
  pub id: String,
  pub x_min: f32,
  pub x_max: f32,
  pub y_min: f32,
  pub y_max: f32,
  pub z_min: f32,
  pub z_max: f32,
  pub mesh_size: f32,
  pub time_step: f32,
}

