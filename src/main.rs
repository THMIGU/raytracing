mod color;
mod hittable;
mod ray;
mod sphere;

use glam::{Vec3, vec3};

use crate::{color::Color, ray::Ray};

const ASPECT_RATIO: f32 = 16_f32 / 9_f32;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

const IMAGE_ASPECT: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
const VIEWPORT_HEIGHT: f32 = 2_f32;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT as f32 * IMAGE_ASPECT;

const FOCAL_LENGTH: u32 = 1;

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
	let oc = center - ray.origin;

	let a = ray.dir.dot(ray.dir);
	let h = ray.dir.dot(oc);
	let c = oc.dot(oc) - radius.powi(2);

	let discriminant = h.powi(2) - a * c;

	if discriminant < 0_f32 {
		return -1_f32;
	}

	(h - discriminant.sqrt()) / a
}

fn ray_color(ray: &Ray) -> Color {
	let sphere_center = vec3(0_f32, 0_f32, -1_f32);

	let t = hit_sphere(sphere_center, 0.5, ray);
	if t > 0_f32 {
		let n = (ray.at(t) - sphere_center).normalize();
		return 0.5 * (n + Vec3::ONE);
	}

	let unit_dir = ray.dir.normalize();
	let a = 0.5 * (unit_dir.y + 1_f32);

	let white = Color::ONE;
	let blue = Color::new(0.5, 0.7, 1_f32);
	let color = white.lerp(blue, a);

	color
}

fn main() {
	// Camera
	let camera_center = Vec3::ZERO;

	// Viewport edges
	let viewport_u = vec3(VIEWPORT_WIDTH, 0_f32, 0_f32);
	let viewport_v = vec3(0_f32, -VIEWPORT_HEIGHT, 0_f32);

	// Pixel deltas
	let pdx_u = viewport_u / IMAGE_WIDTH as f32;
	let pdx_v = viewport_v / IMAGE_HEIGHT as f32;

	// Origin calculation
	let viewport_nw = camera_center
		- vec3(0_f32, 0_f32, FOCAL_LENGTH as f32)
		- viewport_u / 2_f32
		- viewport_v / 2_f32;
	let origin = viewport_nw + 0.5 * (pdx_u + pdx_v);

	// Render
	println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
	for j in 0..IMAGE_HEIGHT {
		eprint!("\rScanlines remaining: {} \r", IMAGE_HEIGHT - j);

		for i in 0..IMAGE_WIDTH {
			let pixel_center = origin + (i as f32 * pdx_u) + (j as f32 * pdx_v);
			let ray_dir = pixel_center - camera_center;

			let r = Ray::new(camera_center, ray_dir);

			let color = ray_color(&r);
			color::write_color(color);
		}
	}

	eprintln!("\rDone.                     ");
}
