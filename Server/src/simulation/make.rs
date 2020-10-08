use crate::simulation::parameters::{Parameters, BoundaryParameters};

pub fn parameters_test1() -> Parameters {
  let mut boundaries = Vec::new();
  boundaries.push(BoundaryParameters {
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
  boundaries.push(BoundaryParameters {
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
  boundaries.push(BoundaryParameters {
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
  boundaries.push(BoundaryParameters {
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
  boundaries.push(BoundaryParameters {
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
  boundaries.push(BoundaryParameters {
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
  Parameters {
      uuid: "test1_uuid01234".to_string(),
      name: "test 1".to_string(),
      project_id: "Project test".to_string(),
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
  }
}


