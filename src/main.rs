mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;

use std::rc::Rc;

use glam::vec3;

use crate::{
	camera::Camera,
	color::Color,
	hittable_list::HittableList,
	material::{Lambertian, Metal},
	sphere::Sphere,
};

fn main() {
	let mut world = HittableList::new();

	let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0_f32)));
	let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
	let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
	let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

	world.add(Sphere::new(vec3(0_f32, -100.5, -1_f32), 100_f32, mat_ground));
	world.add(Sphere::new(vec3(0_f32, 0_f32, -1.2), 0.5, mat_center));
	world.add(Sphere::new(vec3(-1_f32, 0_f32, -1_f32), 0.5, mat_left));
	world.add(Sphere::new(vec3(1_f32, 0_f32, -1_f32), 0.5, mat_right));

	let cam = Camera::new(16_f32 / 9_f32, 400, 128, 50);

	cam.render(&world);
}
