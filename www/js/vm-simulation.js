import { Viewer } from '/jsm/viewer.js'

var app = new Vue({
  el: '#app',
  data: {
    parameters: {
      name: "unnamed",
      id: "",
      x_min: -10.0,
      x_max:  10.0,
      y_min: -10.0,
      y_max:  10.0,
      z_min: -10.0,
      z_max:  10.0,
      mesh_size: 1.0,
      time_step: 1.0
    },
    viewer: null
  },
  computed: {
  },
  mounted: function() {
    this.viewer = new Viewer("threeJsContainer");
    this.viewer.animate();
  },
  methods: {
    start_simulation: function() {
      JdC.HttpRequest("/api/run_simulation", 
           "POST", 
           JSON.stringify(this.parameters), 
           null,
           (res) => {
             console.log("Response to http request to /api/run_simulation: ", res);
           });
    }
  }
});



