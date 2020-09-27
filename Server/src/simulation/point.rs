use serde::{Serialize, Deserialize};

/**
 * Neighbour enum that allow a point to refer to its neightbours or 
 * None if no neighbour
 */
#[derive(Serialize, Deserialize)]
enum Neighbour {
  None,
  Point,
}

/**
 * PointType enum refers to the type of point
 *  Boundary: point represents a boundary condition;
 *  Hanging: point is a hanging node and is calculated by interpolation from its
 *           neighbours;
 *  Solved: point is solved
 */
#[derive(Serialize, Deserialize)]
enum PointType {
  Boundary,
  Hanging,
  Solved,
}

const POINT_IMJK: i32 = 0;
const POINT_IPJK: i32 = 1;
const POINT_IJMK: i32 = 2;
const POINT_IJPK: i32 = 3;
const POINT_IJKM: i32 = 4;
const POINT_IJKP: i32 = 5;

#[derive(Serialize, Deserialize)]
pub struct Point {
  index: i64,
  point_type: PointType,
  neighbours: [Neighbour; 6],
  x: [f64; 3],
  rho: f64,
  u: [f64; 3],
  t: f64,
}

