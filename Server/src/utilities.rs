use std::error::Error;

mod export_to_legacy_vtk;

use crate::simulation::Simulation;
use crate::simulation::parameters::Parameters;
use crate::physicsmodel::PhysicsModel;

pub fn to_legacy_vtk(sim: &Simulation, o:&mut std::fs::File) -> Result<(), Box<dyn Error>> {
  export_to_legacy_vtk::to_legacy_vtk(sim, o)
}

/*
pub fn make_default_parameters() -> Parameters {
  make_default_parameters::make_default_parameters()
}
*/

