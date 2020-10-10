use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::mesh::Mesh;

pub const UX: usize = 1;
pub const UY: usize = 2;
pub const UZ: usize = 3;
pub const P: usize = 4;

/**
 * Physics model - incompressible navier stokes equation at present
 */
#[derive(Serialize, Deserialize)]
pub struct PhysicsModel {
  map: HashMap<Uuid, HashMap<usize, usize>>,
  values: Vec<f64>,
}

impl PhysicsModel {
  /**
   * Constructor
   */
  pub fn new() -> PhysicsModel {
    PhysicsModel {
      map: HashMap::new(),
      values: Vec::new(),
    }
  }

  /**
   * Initialise
   */
  pub fn initialize(&mut self, mesh: &Mesh) {
    mesh.each_point( |uuid, point| {  // println!("{}", uuid) );
       let mut map = HashMap::new();

       map.insert(UX, self.values.len());
       self.values.push(1f64);

       map.insert(UY, self.values.len());
       self.values.push(2f64);

       map.insert(UZ, self.values.len());
       self.values.push(3f64);

       map.insert(P, self.values.len());
       self.values.push(0f64);

       self.map.insert(*uuid, map);
     }
   );
  }

  /**
   * Loop through all scalar variables
   */
  pub fn each_scalar<F: FnMut(&usize)>(&self, mut f: F) {
    f(&P);
  }

  /**
   * Loop through all vector variables
   */
  pub fn each_vector<F: FnMut([&usize; 3])>(&self, mut f: F) {
    f([&UX, &UY, &UZ]);
  }

  /**
   * Retrieve scalar name from scalarId
   */
  pub fn get_scalar_name(&self, scalar_id: &usize) -> String {
    // let mut name = "".to_string();
    match *scalar_id {
      UX => "X Velocity".to_string(),
      UY => "Y Velocity".to_string(),
      UZ => "Z Velocity".to_string(),
      P  => "Pressure".to_string(),
      _ => format!("Variable {}", scalar_id),
    }
  }

  /**
   * Retrieve vector name from scalarIds
   */
  pub fn get_vector_name(&self, scalar_ids: [&usize; 3]) -> String {
    // let mut name = "".to_string();
    match scalar_ids {
      [&UX, &UY, &UZ] => "Velocity".to_string(),
      _  => format!("Variable [{}, {}, {}] unknown", scalar_ids[0], scalar_ids[1], scalar_ids[2]).to_string(),
    }
  }

  /**
   * Get values based on point Id and scalar Id
   */
  pub fn get_value(&self, uuid: &Uuid, variable_id: &usize) -> &f64 {
    &self.values[self.map[uuid][variable_id]]
  }


}
