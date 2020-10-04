import { Viewer } from '/jsm/viewer.js'

var app = new Vue({
  el: '#app',
  data: {
    selected_boundary_uuid: null,
    parameters: {
      uuid: libUtils.uuidv4(),
      name: "unnamed",
      project_id: "",
      x_center: 0.0,
      y_center: 0.0,
      z_center: 0.0,
      x_n_points:  3,
      y_n_points:  3,
      z_n_points:  3,
      mesh_size: 1.0,
      time_step: 1.0,
      n_time_steps: 100,
      norm_velocity: 1.0,
      norm_density: 1.0,
      norm_enthalpy: 1.0,
      boundaries: [
        { uuid: libUtils.uuidv4(),
          name: "Left",
          bctype: "Inlet",
          geometry: null,
          vx: 1.0,
          vy: 0.0,
          vz: 0.0,
          p: 0.0,
          t: 20.0 },
        { uuid: libUtils.uuidv4(),
          name: "Right",
          bctype: "Outlet",
          geometry: null,
          vx: 0.0,
          vy: 0.0,
          vz: 0.0,
          p: 0.0,
          t: 20.0 },
        { uuid: libUtils.uuidv4(),
          name: "Front",
          bctype: "Symmetry",
          geometry: null,
          vx: 0.0,
          vy: 0.0,
          vz: 0.0,
          p: 0.0,
          t: 20.0 },
        { uuid: libUtils.uuidv4(),
          name: "Back",
          bctype: "Symmetry",
          geometry: null,
          vx: 0.0,
          vy: 0.0,
          vz: 0.0,
          p: 0.0,
          t: 20.0 },
        { uuid: libUtils.uuidv4(),
          name: "Top",
          bctype: "Symmetry",
          geometry: null,
          vx: 0.0,
          vy: 0.0,
          vz: 0.0,
          p: 0.0,
          t: 20.0 },
        { uuid: libUtils.uuidv4(),
          name: "Bottom",
          bctype: "Symmetry",
          geometry: null,
          vx: 0.0,
          vy: 0.0,
          vz: 0.0,
          p: 0.0,
          t: 20.0 }
      ]
    },
    viewer: null,
    boundary_types: ["Inlet", "Outlet", "Symmetry", "Wall"]
  },
  computed: {
    boundary_type: {
      get: function() { return this.getSelectedBoundary().bctype; },
      set: function(v) { this.getSelectedBoundary().bctype = v; }
    },
    boundary_vx: {
      get: function() { return this.getSelectedBoundary().vx; },
      set: function(v) { this.getSelectedBoundary().vx = v; }
    },
    boundary_vy: {
      get: function() { return this.getSelectedBoundary().vy; },
      set: function(v) { this.getSelectedBoundary().vy = v; }
    },
    boundary_vz: {
      get: function() { return this.getSelectedBoundary().vz; },
      set: function(v) { this.getSelectedBoundary().vz = v; }
    },
    boundary_p: {
      get: function() { return this.getSelectedBoundary().p; },
      set: function(v) { this.getSelectedBoundary().p = v; }
    },
    boundary_t: {
      get: function() { return this.getSelectedBoundary().t; },
      set: function(v) { this.getSelectedBoundary().t = v; }
    }
  },
  mounted: function() {
    this.viewer = new Viewer("threeJsContainer");
    this.viewer.animate();
    // this.selected_boundary_uuid = this.parameters.boundaries[0].uuid;
  },
  methods: {
    getSelectedBoundary: function() {
      if (this.selected_boundary_uuid === null) { this.selected_boundary_uuid = this.parameters.boundaries[0].uuid; }
      return this.parameters.boundaries.find(b => b.uuid == this.selected_boundary_uuid);
    },
    mark_change: function(evt) {
      console.log("Mark a change with event ", evt);
    },
    start_simulation: function() {
      JdC.HttpRequest("/api/run_simulation", 
           "POST", 
           JSON.stringify(this.parameters), 
           null,
           (res) => {
             console.log("Response to http request to /api/run_simulation: ", res);
           });
    },
    get_section: function() {
      JdC.HttpRequest("/api/get_section", 
           "POST", 
           JSON.stringify({uuid: this.parameters.uuid}), 
           null,
           (res) => {
             console.log("Response to http request to /api/get_section: ", res);
           });
    }
  }
});



