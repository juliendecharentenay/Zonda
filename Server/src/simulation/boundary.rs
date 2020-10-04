use serde::{Serialize, Deserialize};
use uuid::Uuid;

// mod point;
// #[path = "point.rs"] mod point;

#[derive(Serialize, Deserialize)]
pub struct Boundary {
  pub parameters_uuid: String,
  pub point_list: Vec<Uuid>,
}

