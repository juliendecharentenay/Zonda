use std::io::Write;
use uuid::Uuid;
use std::collections::HashMap;
use std::error::Error;

use crate::simulation::Simulation;
use crate::mesh::cell;
use crate::mesh;

/**
 * Output simulation to legacy VTK file format
 */
pub fn to_legacy_vtk(sim: &Simulation, o:&mut std::fs::File) -> Result<(), Box<dyn Error>> {
  o.write(b"# vtk DataFile Version 2.0\n").ok();
  o.write(format!("{}\n", sim.params.name).as_bytes()).ok();
  o.write(b"ASCII\n").ok();
  o.write(b"DATASET UNSTRUCTURED_GRID\n").ok();
  o.write(b"\n").ok();

  // Write list of points
  o.write(format!("POINTS {} float\n",sim.mesh.as_ref().unwrap().point_list.len()).as_bytes()).ok();
  let mut map: HashMap<&Uuid, usize> = HashMap::new(); let mut i: usize = 0;
  let mut p_list: Vec<&Uuid> = Vec::new();
  for (uuid, point) in sim.mesh.as_ref().unwrap().point_list.iter() { 
    o.write(format!("{0:.5} {1:.5} {2:.5}\n", point.x[0], point.x[1], point.x[2]).as_bytes()).ok();
    map.insert(uuid, i); 
    p_list.push(uuid);
    i += 1; 
  }
  o.write(b"\n").ok();
  assert!(i == sim.mesh.as_ref().unwrap().point_list.len());

  // Write cell data
  let mut cells: Vec<&cell::Cell> = Vec::new();
  for c in sim.mesh.as_ref().unwrap().cell_list.values() { if ! c.has_children() { cells.push(c); } }
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
  for uuid in p_list.iter() { o.write(format!("{:.5}\n", sim.mesh.as_ref().unwrap().point_list[uuid].x[0]).as_bytes()).ok(); }
  o.write(b"\n").ok();
 
  o.write(b"SCALARS Y float 1\nLOOKUP_TABLE default\n").ok();
  for uuid in p_list.iter() { o.write(format!("{:.5}\n", sim.mesh.as_ref().unwrap().point_list[uuid].x[1]).as_bytes()).ok(); }
  o.write(b"\n").ok();

  o.write(b"SCALARS Z float 1\nLOOKUP_TABLE default\n").ok();
  for uuid in p_list.iter() { o.write(format!("{:.5}\n", sim.mesh.as_ref().unwrap().point_list[uuid].x[2]).as_bytes()).ok(); }
  o.write(b"\n").ok();

/*

  o.write(b"SCALARS Density float 1\nLOOKUP_TABLE default\n").ok();
  for uuid in p_list.iter() { o.write(format!("{:.5}\n", sim.mesh.as_ref().unwrap().point_list[uuid].rho).as_bytes()).ok(); }
  o.write(b"\n").ok();

  o.write(b"SCALARS Temperature float 1\nLOOKUP_TABLE default\n").ok();
  for uuid in p_list.iter() { o.write(format!("{:.5}\n", sim.mesh.as_ref().unwrap().point_list[uuid].t).as_bytes()).ok(); }
  o.write(b"\n").ok();

  o.write(b"VECTORS Velocity float\n").ok();
  for uuid in p_list.iter() { o.write(format!("{:.5} {:.5} {:.5}\n", sim.mesh.as_ref().unwrap().point_list[uuid].u[0], sim.mesh.as_ref().unwrap().point_list[uuid].u[1], sim.mesh.as_ref().unwrap().point_list[uuid].u[2]).as_bytes()).ok(); }
  o.write(b"\n").ok();
*/

  Ok(())
}


