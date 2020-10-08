use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod boundary;
use boundary::Boundary;
use crate::simulation::parameters::BoundaryParameters;

pub mod cell;
use cell::Cell;
pub mod point;
use point::Point;

#[test]
fn test1() {
  let mut mesh: Mesh = Mesh::new();
  let boundaries: Vec<BoundaryParameters> = Vec::new();
  mesh.initialize(0.0, 0.0, 0.0,
         11, 11, 11,
         0.1, &boundaries);
  assert_eq!(mesh.get_number_of_points(), 11*11*11);
  assert_eq!(mesh.get_number_of_cells(),  10*10*10);
  assert_eq!(mesh.get_range(), ([-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]));
}

#[test]
fn test2() {
  let mut mesh: Mesh = Mesh::new();
  let boundaries: Vec<BoundaryParameters> = Vec::new();
  mesh.initialize(0.0, 0.1, 0.2,
         11, 9, 7,
         0.1, &boundaries);
  assert_eq!(mesh.get_number_of_points(), 11*9*7);
  assert_eq!(mesh.get_number_of_cells(),  10*8*6);
  let (min, max) = mesh.get_range();
  assert!( (((min[0] - -0.5).abs() < 1e-5) &&
            ((min[1] - -0.3).abs() < 1e-5) &&
            ((min[2] - -0.1).abs() < 1e-5) && 
            ((max[0] -  0.5).abs() < 1e-5) && 
            ((max[1] -  0.5).abs() < 1e-5) && 
            ((max[2] -  0.5).abs() < 1e-5))
  );
}

/**
 * Mesh object
 */
#[derive(Serialize, Deserialize)]
pub struct Mesh {
  pub point_list: HashMap<Uuid, Point>,
  pub solved_point_list: Vec<Uuid>,
  pub hanging_point_list: Vec<Uuid>,
  pub boundary_list: HashMap<String, Boundary>,
  pub cell_list: HashMap<Uuid, Cell>,
}

impl Mesh {
  pub fn new() -> Mesh {
    Mesh {
      point_list: HashMap::new(),
      solved_point_list: Vec::new(),
      hanging_point_list: Vec::new(),
      boundary_list: HashMap::new(),
      cell_list: HashMap::new(),
    }
  }

  /**
   * Print some info
   */
  pub fn print_info(&self) {
    println!("Number of points: {}", &self.point_list.len());
    println!("   - mesh statistic:");
    println!("   - number of solved points:  {}", self.solved_point_list.len());
    println!("   - number of hanging points: {}", self.hanging_point_list.len());
  }

  /**
   * Retrieve number of hanging points
   */
  pub fn get_number_of_hanging_points(&self) -> usize {
    self.hanging_point_list.len()
  }

  /**
   * Retrieve number of solved points
   */
  pub fn get_number_of_solved_points(&self) -> usize {
    self.solved_point_list.len()
  }

  /**
   * Retrieve number of points
   */
  pub fn get_number_of_points(&self) -> usize {
    self.point_list.len()
  }

  /**
   * Retrieve number of cells
   */
  pub fn get_number_of_cells(&self) -> usize {
    self.cell_list.len()
  }

  /**
   * Get range: retrieve a tuple with 2 array representing the lower and 
   * upper x, y, z coordinates
   */
  pub fn get_range(&self) -> ([f64; 3], [f64; 3]) {
    let mut ptl: Vec<&Point> = self.point_list.values().collect();
    ptl.sort_by(|a, b| a.index.cmp(&b.index));
    (ptl.first().unwrap().x.clone(), ptl.last().unwrap().x.clone())
  }

 /**
   * Initialise the simulation:
   *     Create the mesh point and cells from the input parameters
   *     Populate simulation related parameters:
   *       - assign solved point;
   *       - assign boundary condition parameters and points [for edges];
   */
  pub fn initialize(&mut self, x_center: f64, y_center: f64, z_center: f64,
                    x_np: usize, y_np: usize, z_np: usize,
                    mesh_size: f64,
                    boundaries: &Vec<BoundaryParameters> ) {
    println!("Simulation::make_mesh - start");

    // Generate points
    let mut id_list: Vec<Uuid> = Vec::new();
    for k in 0..z_np {
      for j in 0..y_np {
        for i in 0..x_np {
          let index = i + x_np*(j + y_np * k);
          let p: Point = Point::new(index,
                  [f64::from(x_center + mesh_size * (i as f64 - (x_np-1) as f64 /2.0)),
                   f64::from(y_center + mesh_size * (j as f64 - (y_np-1) as f64 /2.0)),
                   f64::from(z_center + mesh_size * (k as f64 - (z_np-1) as f64 /2.0))]);
          let p_id: Uuid = *p.get_id();
          self.point_list.insert(p_id, p);
          id_list.push(p_id);
        }
      }
    }

    // Assign point neighbours
    for k in 0..z_np {
      for j in 0..y_np {
        for i in 0..x_np {
          let index = i + x_np * (j + y_np * k);
          let id: &Uuid = &id_list[index];
          let p: &mut point::Point = self.point_list.get_mut(id).unwrap();
          if i > 0 { p.set_neighbour(point::IMJK, &id_list[index-1]); }
          if j > 0 { p.set_neighbour(point::IJMK, &id_list[index-x_np]); }
          if k > 0 { p.set_neighbour(point::IJKM, &id_list[index-x_np*y_np]); }
          if i < x_np-1 { p.set_neighbour(point::IPJK, &id_list[index+1]); }
          if j < y_np-1 { p.set_neighbour(point::IJPK, &id_list[index+x_np]); }
          if k < z_np-1 { p.set_neighbour(point::IJKP, &id_list[index+x_np*y_np]); }
        }
      }
    }

    // Assign cells
    for k in 0..z_np-1 {
      for j in 0..y_np-1 {
        for i in 0..x_np-1 {
          let index = i + x_np * (j + y_np * k);
          let c: cell::Cell = cell::make_cell(&id_list[index], &id_list[index+1], &id_list[index+1+x_np], &id_list[index+x_np], &id_list[index+x_np*y_np], &id_list[index+1+x_np*y_np], &id_list[index+1+x_np+x_np*y_np], &id_list[index+x_np+x_np*y_np]);
          self.cell_list.insert(*c.get_id(), c);
        }
      }
    }

    // Assign list of points to be solved
    for k in 1..z_np-1 {
      for j in 1..y_np-1 {
        for i in 1..x_np-1 {
          let index = i + x_np * (j + y_np * k);
          self.solved_point_list.push(id_list[index]);
        }
      }
    }

    // Assign boundary condition parameters and point list
    for b in boundaries.iter() {
      println!("Handling boundary {} ", b.name);
      let mut boundary = boundary::Boundary {
              point_list: Vec::new()
              };
       
       if b.name == "Left" {
         for k in 1..z_np-1 {
           for j in 1..y_np-1 {
             let index = 0 + x_np * (j + y_np * k);
             boundary.point_list.push(id_list[index]);
           }
         }
       } else if b.name == "Right" {
         for k in 1..z_np-1 {
           for j in 1..y_np-1 {
             let index = x_np-1 + x_np * (j + y_np * k);
             boundary.point_list.push(id_list[index]);
           }
         }
       } else if b.name == "Front" {
         for k in 1..z_np-1 {
           for i in 1..x_np-1 {
             let index = i + x_np * (0 + y_np * k);
             boundary.point_list.push(id_list[index]);
           }
         }
       } else if b.name == "Back" {
         for k in 1..z_np-1 {
           for i in 1..x_np-1 {
             let index = i + x_np * (y_np-1 + y_np * k);
             boundary.point_list.push(id_list[index]);
           }
         }
       } else if b.name == "Bottom" {
         for j in 1..y_np-1 {
           for i in 1..x_np-1 {
             let index = i + x_np * (j + y_np * 0);
             boundary.point_list.push(id_list[index]);
           }
         }
       } else if b.name == "Top" {
         for j in 1..y_np-1 {
           for i in 1..x_np-1 {
             let index = i + x_np * (j + y_np * (z_np-1));
             boundary.point_list.push(id_list[index]);
           }
         }
       }
       self.boundary_list.insert(b.uuid.clone(), boundary);
    }
 
    // Assign corner points to hanging points
    {
      self.hanging_point_list.push(id_list[ 0      + x_np * (0      + y_np * 0       ) ]);
      self.hanging_point_list.push(id_list[ x_np-1 + x_np * (0      + y_np * 0       ) ]);
      self.hanging_point_list.push(id_list[ 0      + x_np * (y_np-1 + y_np * 0       ) ]);
      self.hanging_point_list.push(id_list[ x_np-1 + x_np * (y_np-1 + y_np * 0       ) ]);
      self.hanging_point_list.push(id_list[ 0      + x_np * (0      + y_np * (z_np-1)) ]);
      self.hanging_point_list.push(id_list[ x_np-1 + x_np * (0      + y_np * (z_np-1)) ]);
      self.hanging_point_list.push(id_list[ 0      + x_np * (y_np-1 + y_np * (z_np-1)) ]);
      self.hanging_point_list.push(id_list[ x_np-1 + x_np * (y_np-1 + y_np * (z_np-1)) ]);
    }

    for i in 1..x_np-1 {
      self.hanging_point_list.push(id_list[ i + x_np * (0      + y_np * 0      ) ]);
      self.hanging_point_list.push(id_list[ i + x_np * (y_np-1 + y_np * 0      ) ]);
      self.hanging_point_list.push(id_list[ i + x_np * (0      + y_np * (z_np-1)) ]);
      self.hanging_point_list.push(id_list[ i + x_np * (y_np-1 + y_np * (z_np-1)) ]);
    }
    for j in 1..y_np-1 {
      self.hanging_point_list.push(id_list[ 0      + x_np * (j + y_np * 0      ) ]);
      self.hanging_point_list.push(id_list[ x_np-1 + x_np * (j + y_np * 0      ) ]);
      self.hanging_point_list.push(id_list[ 0      + x_np * (j + y_np * (z_np-1)) ]);
      self.hanging_point_list.push(id_list[ x_np-1 + x_np * (j + y_np * (z_np-1)) ]);
    }
    for k in 1..z_np-1 {
      self.hanging_point_list.push(id_list[ 0      + x_np * (0      + y_np * k) ]);
      self.hanging_point_list.push(id_list[ x_np-1 + x_np * (0      + y_np * k) ]);
      self.hanging_point_list.push(id_list[ 0      + x_np * (y_np-1 + y_np * k) ]);
      self.hanging_point_list.push(id_list[ x_np-1 + x_np * (y_np-1 + y_np * k) ]);
    }

    println!(" - complete with {} points", self.point_list.len());
  }

}
