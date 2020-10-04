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
  id: Uuid,
  neighbours: [Option<Uuid>; 6],
  pub x: [f64; 3],
  pub rho: f64,
  pub u: [f64; 3],
  pub t: f64,
}

pub fn make_point(x: [f64; 3]) -> Point {
  Point {
    id: Uuid::new_v4(),
    neighbours: [None; 6],
    x,
    rho: 1.20,
    u: [0.0; 3],
    t: 295.0
  }
}

impl Point {
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


