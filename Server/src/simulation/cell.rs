use serde::{Serialize, Deserialize};
use uuid::Uuid;

const IJK: usize    = 0;
const IPJK: usize   = 1;
const IPJPK: usize  = 2;
const IJPK: usize   = 3;
const IJKP: usize   = 4;
const IPJKP: usize  = 5;
const IPJPKP: usize = 6;
const IJPKP: usize  = 7;

pub fn make_cell(ijk: &Uuid, ipjk: &Uuid, ipjpk: &Uuid, ijpk: &Uuid,
              ijkp: &Uuid, ipjkp: &Uuid, ipjpkp: &Uuid, ijpkp: &Uuid) -> Cell {
 Cell {
   id: Uuid::new_v4(),
   point_list: [*ijk, *ipjk, *ipjpk, *ijpk, *ijkp, *ipjkp, *ipjpkp, *ijpkp],
   children_list: Vec::new(),
 }
}

#[derive(Serialize, Deserialize)]
pub struct Cell {
  id: Uuid,
  point_list: [Uuid; 8],
  children_list: Vec<Uuid>,
}

impl Cell {
  /*
   * retrieve id
   */
  pub fn get_id(&self) -> &Uuid {
    &self.id
  }
}

