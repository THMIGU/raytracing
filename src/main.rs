mod color;
mod ray;

use glam::{Vec3, vec3};

use crate::{color::Color, ray::Ray};

const ASPECT_RATIO: f32 = 16_f32 / 9_f32;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

const IMAGE_ASPECT: f32 = IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32;
const VIEWPORT_HEIGHT: u32 = 2;
const VIEWPORT_WIDTH: u32 = (VIEWPORT_HEIGHT as f32 * IMAGE_ASPECT) as u32;

const FOCAL_LENGTH: u32 = 1;

fn ray_color(ray: &Ray) -> Color {
	Color::ZERO
}

fn main() {
	// Camera
	let camera_center = Vec3::ZERO;

	// Viewport edges
	let viewport_u = vec3(VIEWPORT_WIDTH as f32, 0_f32, 0_f32);
	let viewport_v = vec3(0_f32, -(VIEWPORT_HEIGHT as f32), 0_f32);

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
