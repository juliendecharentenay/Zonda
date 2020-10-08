use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub const IMJK: usize = 0;
pub const IPJK: usize = 1;
pub const IJMK: usize = 2;
pub const IJPK: usize = 3;
pub const IJKM: usize = 4;
pub const IJKP: usize = 5;

#[derive(Serialize, Deserialize)]
pub struct Point {
  pub id: Uuid,
  pub index: usize,
  pub neighbours: [Option<Uuid>; 6],
  pub x: [f64; 3],
}

impl Point {
  /**
   * constructor
   */
  pub fn new(index: usize, x: [f64; 3]) -> Point {
    Point {
      id: Uuid::new_v4(),
      index,
      neighbours: [None; 6],
      x,
    }
  }

  /*
   * retrieve id
   */
  pub fn get_id(&self) -> &Uuid {
    &self.id
  }

  /*
   * set neighbour
   */
  pub fn set_neighbour(&mut self, i: usize, neighbour_id: &Uuid) -> &mut Point {
    self.neighbours[i] = Some(*neighbour_id);
    self
  }
}


