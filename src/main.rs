mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;

use glam::vec3;

use crate::{camera::Camera, hittable_list::HittableList, sphere::Sphere};

fn main() {
	let mut world = HittableList::new();
	world.add(Sphere::new(vec3(0_f32, 0_f32, -1_f32), 0.5));
	world.add(Sphere::new(vec3(0_f32, -100.5, -1_f32), 100_f32));

	let cam = Camera::new(16_f32 / 9_f32, 400, 32, 50);

	cam.render(&world);
}
