use serde::{Serialize, Deserialize};

#[path = "point.rs"] mod point;

const CELL_IJK: i32    = 0;
const CELL_IPJK: i32   = 1;
const CELL_IPJPK: i32  = 2;
const CELL_IJPK: i32   = 3;
const CELL_IJKP: i32   = 4;
const CELL_IPJKP: i32  = 5;
const CELL_IPJPKP: i32 = 6;
const CELL_IJPKP: i32  = 7;

#[derive(Serialize, Deserialize)]
pub struct Cell {
  point_index_list: [i64; 8],
  children_index_list: Vec<i64>,
}

