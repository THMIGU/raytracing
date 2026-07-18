mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;

use std::{f32::consts::PI, rc::Rc};

use glam::{Vec3, vec3};

use crate::{
	camera::Camera,
	color::Color,
	hittable_list::HittableList,
	material::{Dielectric, Lambertian, Metal},
	sphere::Sphere,
};

fn main() {
	let mut world = HittableList::new();

	let r = (PI / 4_f32).cos();

	let mat_left = Rc::new(Lambertian::new(Color::new(0_f32, 0_f32, 1_f32)));
	let mat_right = Rc::new(Lambertian::new(Color::new(1_f32, 0_f32, 0_f32)));

	world.add(Sphere::new(vec3(-r, 0_f32, -1_f32), r, mat_left));
	world.add(Sphere::new(vec3(r, 0_f32, -1_f32), r, mat_right));

	let cam = Camera::new(16_f32 / 9_f32, 400, 128, 64, 90_f32);
	cam.render(&world);
}
