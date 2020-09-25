
var app = new Vue({
  el: '#app',
  data: {
    simulation: {
      name: "unnamed",
      id: ""
    },
    domain: {
      xMin: -10.0,
      xMax:  10.0,
      yMin: -10.0,
      yMax:  10.0,
      zMin: -10.0,
      zMax:  10.0
    },
    parameters: {
      meshSize: 1.0,
      timeStep: 1.0
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
  }
});



