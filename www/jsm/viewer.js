import * as THREE from '/libm/threejs/three.module.js';
import { TrackballControls } from '/libm/threejs/TrackballControls.js';


class Viewer {
  constructor(containerId) {
    var container = document.getElementById(containerId);

    this.renderer = new THREE.WebGLRenderer();
    this.renderer.setSize(container.clientWidth, container.clientHeight);
    container.appendChild(this.renderer.domElement);

    this.scene = new THREE.Scene();
    this.camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    this.camera.position.z = 5; // set(100, -400, 2000);
    this.scene.add(this.camera);

    this.controls = new TrackballControls(this.camera, this.renderer.domElement);
    // this.controls.minDistance = 200;
    // this.controls.maxDistance = 500;

    this.geometry = new THREE.BoxGeometry();
    var material = new THREE.MeshBasicMaterial( { color: 0x00ff00 } );
    this.cube = new THREE.Mesh(this.geometry, material);
    this.scene.add(this.cube);
  }

  animate() {
    requestAnimationFrame( this.animate.bind(this) );
    this.controls.update();
    this.renderer.render(this.scene, this.camera);
  }
}

export { Viewer };
