use serde::{Serialize, Deserialize};
use std::io::Write;
use std::io::Read;
use std::collections::HashMap;
use uuid::Uuid;

#[path = "parameters.rs"] mod parameters;
#[path = "point.rs"] mod point;
#[path = "boundary.rs"] mod boundary;
#[path = "cell.rs"] mod cell;
#[path = "segment.rs"] mod segment;

pub fn make_simulation(params: parameters::Parameters) -> Simulation {
  Simulation {
    params,
    point_list: HashMap::new(),
    solved_point_list: Vec::new(),
    hanging_point_list: Vec::new(),
    boundary_list: Vec::new(),
    cell_list: HashMap::new(),
    segment_list: Vec::new(),
  }
}

pub fn read_simulation(f:&mut std::fs::File) -> Simulation {
  let mut c = String::new();
  f.read_to_string(&mut c).unwrap();
  serde_json::from_str(&c).unwrap()
}

/**
 * Simulation object containing all simulation information
 */
#[derive(Serialize, Deserialize)]
pub struct Simulation {
  params: parameters::Parameters,
  point_list: HashMap<Uuid, point::Point>,
  solved_point_list: Vec<Uuid>,
  hanging_point_list: Vec<Uuid>,
  boundary_list: Vec<boundary::Boundary>,
  cell_list: HashMap<Uuid, cell::Cell>,
  segment_list: Vec<segment::Segment>,
}

impl Simulation {
  /**
   * Print some info
   */
  pub fn print_info(&self) {
    println!("Simulation {}/project {} has uuid {}", &self.params.name, &self.params.project_id, &self.params.uuid);
    println!("Number of points: {}", &self.point_list.len());
  }

  /**
   * Accessor function to retrieve uuid
   */
  pub fn get_uuid(&self) -> &String {
    &self.params.uuid
  }

  /**
   * Create the mesh from the input parameters
   */
  pub fn make_mesh(&mut self) {
    println!("Simulation::make_mesh - start");
    let x_np = self.params.x_n_points;
    let y_np = self.params.y_n_points;
    let z_np = self.params.z_n_points;

    // Generate points
    let mut id_list: Vec<Uuid> = Vec::new();
    for k in 0..z_np {
      for j in 0..y_np {
        for i in 0..x_np {
          let p: point::Point = point::make_point([f64::from(self.params.x_center + self.params.mesh_size * (i as f64 - (x_np-1) as f64 /2.0)),
                   f64::from(self.params.y_center + self.params.mesh_size * (j as f64 - (y_np-1) as f64 /2.0)),
                   f64::from(self.params.z_center + self.params.mesh_size * (k as f64 - (z_np-1) as f64 /2.0))]);
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
          let index = i + x_np * (j + z_np * k);
          let id: &Uuid = &id_list[index];
          let p: &mut point::Point = self.point_list.get_mut(id).unwrap();
          if i > 0 { p.set_neighbour(point::IMJK, &id_list[index-1]); }
          if j > 0 { p.set_neighbour(point::IJMK, &id_list[index-x_np]); }
          if k > 0 { p.set_neighbour(point::IJKM, &id_list[index-x_np*y_np]); }
          if i < x_np-1 { p.set_neighbour(point::IPJK, &id_list[index+1]); }
          if j < y_np-1 { p.set_neighbour(point::IJPK, &id_list[index+x_np]); }
          if k < z_np-1 { p.set_neighbour(point::IJPK, &id_list[index+x_np*y_np]); }
        }
      }
    }

    // Assign cells
    for k in 0..z_np-1 {
      for j in 0..y_np-1 {
        for i in 0..x_np-1 {
          let index = i + x_np * (j + z_np * k);
          let c: cell::Cell = cell::make_cell(&id_list[index], &id_list[index+1], &id_list[index+1+x_np], &id_list[index+x_np], &id_list[index+x_np*y_np], &id_list[index+1+x_np*y_np], &id_list[index+1+x_np+x_np*y_np], &id_list[index+x_np+x_np*y_np]);
          self.cell_list.insert(*c.get_id(), c);
        }
      }
    }

    // Assign boundary list


    println!("Simulation::make_mesh - complete with {} points", self.point_list.len());
  }

  /**
   * Output simulation to object implementing Write trait
   */
  pub fn output(&self, o:&mut std::fs::File) {
    o.write_all(serde_json::to_string(&self).unwrap().as_bytes()).unwrap();
  }
}

