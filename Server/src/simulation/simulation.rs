use serde::{Serialize, Deserialize};

#[path = "parameters.rs"] mod parameters;
#[path = "point.rs"] mod point;
#[path = "boundary.rs"] mod boundary;
#[path = "cell.rs"] mod cell;
#[path = "segment.rs"] mod segment;

/**
 * Simulation object containing all simulation information
 */
#[derive(Serialize, Deserialize)]
pub struct Simulation {
  params: parameters::Parameters,
  point_list: Vec<point::Point>,
  solved_point_list: Vec<i64>,
  hanging_point_list: Vec<i64>,
  boundary_list: Vec<boundary::Boundary>,
  cell_list: Vec<cell::Cell>,
  segment_list: Vec<segment::Segment>,
}

