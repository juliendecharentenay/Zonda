use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Boundary {
  pub point_list: Vec<Uuid>,
}

