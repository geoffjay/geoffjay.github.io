import * as THREE from "three";

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
const geometry = new THREE.BoxGeometry(1, 1, 1);
const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
const cube = new THREE.Mesh(geometry, material);

export function init() {
  const canvas = document.getElementById("canvas");

  // renderer.setSize(window.innerWidth, window.innerHeight);
  renderer.setSize(800, 600);
  canvas.appendChild( renderer.domElement );

  scene.add(cube);

  camera.position.z = 5;

  renderer.render(scene, camera);
}

export function render() {
  requestAnimationFrame(render);

	cube.rotation.x += 0.0001;
	cube.rotation.y += 0.0001;

	renderer.render(scene, camera);
}
