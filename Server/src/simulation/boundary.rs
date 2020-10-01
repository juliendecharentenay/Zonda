use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[path = "point.rs"] mod point;

#[derive(Serialize, Deserialize)]
pub struct BoundaryParameters {
  name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Boundary {
  parms: BoundaryParameters,
  point_index_list: Vec<Uuid>,
}

