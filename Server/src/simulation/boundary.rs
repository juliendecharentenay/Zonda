use serde::{Serialize, Deserialize};

#[path = "point.rs"] mod point;

#[derive(Serialize, Deserialize)]
pub struct BoundaryParameters {
  name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Boundary {
  parms: BoundaryParameters,
  point_index_list: Vec<i64>,
}

