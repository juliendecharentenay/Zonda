use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[path = "point.rs"] mod point;

#[derive(Serialize, Deserialize)]
pub struct Segment {
  start_point_id: Uuid,
  end_point_id: Uuid,
}

