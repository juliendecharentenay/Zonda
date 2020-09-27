use serde::{Serialize, Deserialize};

#[path = "point.rs"] mod point;

#[derive(Serialize, Deserialize)]
pub struct Segment {
  start_point_index: i64,
  end_point_index: i64,
}

