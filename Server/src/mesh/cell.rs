use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub const IJK: usize    = 0;
pub const IPJK: usize   = 1;
pub const IPJPK: usize  = 2;
pub const IJPK: usize   = 3;
pub const IJKP: usize   = 4;
pub const IPJKP: usize  = 5;
pub const IPJPKP: usize = 6;
pub const IJPKP: usize  = 7;

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

  /*
   * Retrun whether a cell has children or not
   */
  pub fn has_children(&self) -> bool {
    self.children_list.len() > 0
  }

  /*
   * Get list of points
   */
  pub fn get_points(&self) -> &[Uuid] {
    &self.point_list
  }
}

