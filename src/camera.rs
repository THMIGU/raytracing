use glam::{Vec2, Vec3, vec2, vec3};

use crate::{
	color::{self, Color},
	hittable::{HitRecord, Hittable},
	interval::Interval,
	ray::Ray,
	utils::{INFINITY, degrees_to_radians, random_float},
};

pub struct Camera {
	pub aspect_ratio: f32,
	pub image_width: u32,
	pub samples_per_pixel: u32,
	pub max_depth: u32,
	pub vfov: f32,
	image_height: u32,
	center: Vec3,
	p00_loc: Vec3,
	pxd_u: Vec3,
	pxd_v: Vec3,
}

impl Camera {
	fn ray_color(ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
		if depth <= 0 {
			return Color::ZERO;
		}

		let mut rec = HitRecord::new();
		if world.hit(ray, Interval::new(0.01, INFINITY), &mut rec) {
			let mut scattered = Ray::default();
			let mut attenuation = Color::ZERO;

			if rec
				.mat
				.scatter(ray, &rec, &mut attenuation, &mut scattered)
			{
				return attenuation * Self::ray_color(&scattered, depth - 1, world);
			}
			return Color::ZERO;
		}

		let unit_dir = ray.dir.normalize();
		let a = 0.5 * (unit_dir.y + 1_f32);

		let white = Color::ONE;
		let blue = Color::new(0.5, 0.7, 1_f32);
		let color = white.lerp(blue, a);

		color
	}

	fn sample_square() -> Vec2 {
		vec2(random_float(None) - 0.5, random_float(None) - 0.5)
	}

	fn get_ray(&self, i: u32, j: u32) -> Ray {
		let offset = Self::sample_square();
		let pixel_sample = self.p00_loc
			+ ((i as f32 + offset.x) * self.pxd_u)
			+ ((j as f32 + offset.y) * self.pxd_v);

		let ray_origin = self.center;
		let ray_dir = pixel_sample - ray_origin;

		Ray::new(ray_origin, ray_dir)
	}

	pub fn new(
		aspect_ratio: f32,
		image_width: u32,
		samples_per_pixel: u32,
		max_depth: u32,
		vfov: f32,
	) -> Self {
		let mut image_height = (image_width as f32 / aspect_ratio) as u32;
		if image_height < 1 {
			image_height = 1;
		}

		let camera_center = Vec3::ZERO;
		let focal_length = 1_f32;

		let theta = degrees_to_radians(vfov);
		let h = (theta / 2_f32).tan();

		let viewport_height = 2_f32 * h * focal_length;

		let image_aspect = image_width as f32 / image_height as f32;
		let viewport_width = viewport_height as f32 * image_aspect;

		let viewport_u = vec3(viewport_width, 0_f32, 0_f32);
		let viewport_v = vec3(0_f32, -viewport_height, 0_f32);

		let pxd_u = viewport_u / image_width as f32;
		let pxd_v = viewport_v / image_height as f32;

		let viewport_nw = camera_center
			- vec3(0_f32, 0_f32, focal_length)
			- viewport_u / 2_f32
			- viewport_v / 2_f32;
		let p00_loc = viewport_nw + 0.5 * (pxd_u + pxd_v);

		Self {
			aspect_ratio,
			image_width,
			samples_per_pixel,
			max_depth,
			vfov,
			image_height,
			center: camera_center,
			p00_loc,
			pxd_u,
			pxd_v,
		}
	}

	pub fn render(&self, world: &impl Hittable) {
		println!("P3\n{} {}\n255", self.image_width, self.image_height);
		for j in 0..self.image_height {
			eprint!("\rScanlines remaining: {} \r", self.image_height - j);

			for i in 0..self.image_width {
				let mut color = Color::ZERO;

				for _ in 0..self.samples_per_pixel {
					let ray = self.get_ray(i, j);
					color += Self::ray_color(&ray, self.max_depth, world);
				}

				color::write_color(color / self.samples_per_pixel as f32);
			}
		}

		eprintln!("\rDone.                     ");
	}
}
