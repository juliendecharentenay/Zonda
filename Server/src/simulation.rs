use serde::{Serialize, Deserialize};
use std::io::Write;
use std::io::Read;
use std::collections::HashMap;
use uuid::Uuid;

// #[path = "parameters.rs"] mod parameters;
// #[path = "point.rs"] mod point;
// #[path = "boundary.rs"] mod boundary;
// #[path = "cell.rs"] mod cell;
// #[path = "segment.rs"] mod segment;

mod parameters;
mod point;
mod boundary;
mod cell;
mod segment;

pub fn make_default_simulation() -> Simulation {
  let mut boundaries = Vec::new();
  boundaries.push(parameters::BoundaryParameters {
       uuid: "111".to_string(),
       name: "Left".to_string(),
       bctype: "Inlet".to_string(),
       geometry: None,
       vx: 1.0,
       vy: 0.0,
       vz: 0.0,
       p: 0.0,
       t: 20.0,
  });
  boundaries.push(parameters::BoundaryParameters {
       uuid: "112".to_string(),
       name: "Right".to_string(),
       bctype: "Outlet".to_string(),
       geometry: None,
       vx: 1.0,
       vy: 0.0,
       vz: 0.0,
       p: 0.0,
       t: 20.0,
  });
  boundaries.push(parameters::BoundaryParameters {
       uuid: "113".to_string(),
       name: "Front".to_string(),
       bctype: "Symmetry".to_string(),
       geometry: None,
       vx: 1.0,
       vy: 0.0,
       vz: 0.0,
       p: 0.0,
       t: 20.0,
  });
  boundaries.push(parameters::BoundaryParameters {
       uuid: "114".to_string(),
       name: "Back".to_string(),
       bctype: "Symmetry".to_string(),
       geometry: None,
       vx: 1.0,
       vy: 0.0,
       vz: 0.0,
       p: 0.0,
       t: 20.0,
  });
  boundaries.push(parameters::BoundaryParameters {
       uuid: "115".to_string(),
       name: "Bottom".to_string(),
       bctype: "Symmetry".to_string(),
       geometry: None,
       vx: 1.0,
       vy: 0.0,
       vz: 0.0,
       p: 0.0,
       t: 20.0,
  });
  boundaries.push(parameters::BoundaryParameters {
       uuid: "116".to_string(),
       name: "Top".to_string(),
       bctype: "Symmetry".to_string(),
       geometry: None,
       vx: 1.0,
       vy: 0.0,
       vz: 0.0,
       p: 0.0,
       t: 20.0,
  });
  Simulation {
    params: parameters::Parameters {
      uuid: "default_simulation".to_string(),
      name: "test".to_string(),
      project_id: "test_project".to_string(),
      x_center: 0.0,
      y_center: 0.0,
      z_center: 0.0,
      x_n_points: 11,
      y_n_points: 7,
      z_n_points: 3,
      mesh_size: 0.1,
      time_step: 0.1,
      n_time_steps: 2,
      norm_velocity: 1.0,
      norm_density: 1.0,
      norm_enthalpy: 1.0,
      boundaries: boundaries,
    },
    point_list: HashMap::new(),
    solved_point_list: Vec::new(),
    hanging_point_list: Vec::new(),
    boundary_list: Vec::new(),
    cell_list: HashMap::new(),
    // segment_list: Vec::new(),
  }
}

pub fn make_simulation(params: parameters::Parameters) -> Simulation {
  Simulation {
    params,
    point_list: HashMap::new(),
    solved_point_list: Vec::new(),
    hanging_point_list: Vec::new(),
    boundary_list: Vec::new(),
    cell_list: HashMap::new(),
    // segment_list: Vec::new(),
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
  // segment_list: Vec<segment::Segment>,
}

impl Simulation {
  /**
   * Print some info
   */
  pub fn print_info(&self) {
    println!("Simulation {}/project {} has uuid {}", &self.params.name, &self.params.project_id, &self.params.uuid);
    println!("Number of points: {}", &self.point_list.len());
    println!("   - mesh statistic:");
    println!("   - number of solved points:  {}", self.solved_point_list.len());
    println!("   - number of hanging points: {}", self.hanging_point_list.len());
    for b in self.boundary_list.iter() {
      let p = self.params.boundaries.iter().find(|&bp| bp.uuid == b.parameters_uuid).unwrap();
      println!("   - number of points in boundary {}: {}", p.name, b.point_list.len());
    }
  }

  /**
   * Accessor function to retrieve uuid
   */
  pub fn get_uuid(&self) -> &String {
    &self.params.uuid
  }

  /**
   * Accessor function to retrieve parameters
   */
  pub fn get_parameters(&self) -> &parameters::Parameters {
    &self.params
  }

  /**
   * Initialise the simulation:
   *     Create the mesh point and cells from the input parameters
   *     Populate simulation related parameters:
   *       - assign solved point;
   *       - assign boundary condition parameters and points [for edges];
   */
  pub fn initialize(&mut self) {
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
    for b in self.params.boundaries.iter() {
      println!("Handling boundary {} ", b.name);
      let mut boundary = boundary::Boundary {
              parameters_uuid: b.uuid.clone(),
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
       self.boundary_list.push(boundary);
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


    println!("Simulation::make_mesh - complete with {} points", self.point_list.len());
  }

  /**
   * Output simulation to object implementing Write trait
   */
  pub fn output(&self, o:&mut std::fs::File) -> std::io::Result<()> {
    o.write_all(serde_json::to_string(&self).unwrap().as_bytes()).unwrap();
    Ok(())
  }

  /**
   * Output simulation to legacy VTK file format
   */
  pub fn to_legacy_vtk(&self, o:&mut std::fs::File) -> std::io::Result<()> {
    o.write(b"# vtk DataFile Version 2.0\n").ok();
    o.write(format!("{}\n", self.params.name).as_bytes()).ok();
    o.write(b"ASCII\n").ok();
    o.write(b"DATASET UNSTRUCTURED_GRID\n").ok();
    o.write(b"\n").ok();

    // Write list of points
    o.write(format!("POINTS {} float\n",self.point_list.len()).as_bytes()).ok();
    let mut map: HashMap<&Uuid, usize> = HashMap::new(); let mut i: usize = 0;
    let mut p_list: Vec<&Uuid> = Vec::new();
    for (uuid, point) in self.point_list.iter() { 
      o.write(format!("{0:.5} {1:.5} {2:.5}\n", point.x[0], point.x[1], point.x[2]).as_bytes()).ok();
      map.insert(uuid, i); 
      p_list.push(uuid);
      i += 1; 
    }
    o.write(b"\n").ok();
    assert!(i == self.point_list.len());

    // Write cell data
    let mut cells: Vec<&cell::Cell> = Vec::new();
    for c in self.cell_list.values() { if ! c.has_children() { cells.push(c); } }
    o.write(format!("CELLS {0} {1}\n", cells.len(), cells.len()*(1 + 8)).as_bytes()).ok();
    for c in cells.iter() {
      o.write(b"8").ok();
      let pts: &[Uuid] = c.get_points();
      let pts2: [&Uuid; 8] = [&pts[cell::IJK], &pts[cell::IPJK], &pts[cell::IJPK], &pts[cell::IPJPK],
                      &pts[cell::IJKP], &pts[cell::IPJKP], &pts[cell::IJPKP], &pts[cell::IPJPKP]];
      for p in pts2.iter() { o.write(format!(" {}", map[p]).as_bytes()).ok(); }
      o.write(b"\n").ok();
    }
    o.write(b"\n").ok();

    // Write cell types
    o.write(format!("CELL_TYPES {}\n", cells.len()).as_bytes()).ok();
    for _c in cells.iter() {o.write(b"11\n").ok();}
    o.write(b"\n").ok();

    // Write data
    o.write(format!("POINT_DATA {}\n", p_list.len()).as_bytes()).ok();

    o.write(b"SCALARS X float 1\nLOOKUP_TABLE default\n").ok();
    for uuid in p_list.iter() { o.write(format!("{:.5}\n", self.point_list[uuid].x[0]).as_bytes()).ok(); }
    o.write(b"\n").ok();
 
    o.write(b"SCALARS Y float 1\nLOOKUP_TABLE default\n").ok();
    for uuid in p_list.iter() { o.write(format!("{:.5}\n", self.point_list[uuid].x[1]).as_bytes()).ok(); }
    o.write(b"\n").ok();

    o.write(b"SCALARS Z float 1\nLOOKUP_TABLE default\n").ok();
    for uuid in p_list.iter() { o.write(format!("{:.5}\n", self.point_list[uuid].x[2]).as_bytes()).ok(); }
    o.write(b"\n").ok();

    o.write(b"SCALARS Density float 1\nLOOKUP_TABLE default\n").ok();
    for uuid in p_list.iter() { o.write(format!("{:.5}\n", self.point_list[uuid].rho).as_bytes()).ok(); }
    o.write(b"\n").ok();

    o.write(b"SCALARS Temperature float 1\nLOOKUP_TABLE default\n").ok();
    for uuid in p_list.iter() { o.write(format!("{:.5}\n", self.point_list[uuid].t).as_bytes()).ok(); }
    o.write(b"\n").ok();

    o.write(b"VECTORS Velocity float\n").ok();
    for uuid in p_list.iter() { o.write(format!("{:.5} {:.5} {:.5}\n", self.point_list[uuid].u[0], self.point_list[uuid].u[1], self.point_list[uuid].u[2]).as_bytes()).ok(); }
    o.write(b"\n").ok();

    Ok(())
  }

}
