use glam::{Vec3, vec3};

use crate::{
	color::{self, Color},
	hittable::{HitRecord, Hittable},
	interval::Interval,
	ray::Ray,
	utils::INFINITY,
};

pub struct Camera {
	pub aspect_ratio: f32,
	pub image_width: u32,
	image_height: u32,
	center: Vec3,
	p00_loc: Vec3,
	pxd_u: Vec3,
	pxd_v: Vec3,
}

impl Camera {
	fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
		let mut rec = HitRecord::new();
		if world.hit(ray, Interval::new(0_f32, INFINITY), &mut rec) {
			return 0.5 * (rec.normal + Color::ONE);
		}

		let unit_dir = ray.dir.normalize();
		let a = 0.5 * (unit_dir.y + 1_f32);

		let white = Color::ONE;
		let blue = Color::new(0.5, 0.7, 1_f32);
		let color = white.lerp(blue, a);

		color
	}

	pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
		let mut image_height = (image_width as f32 / aspect_ratio) as u32;
		if image_height < 1 {
			image_height = 1;
		}

		let camera_center = Vec3::ZERO;
		let focal_length = 1_f32;

		let image_aspect = image_width as f32 / image_height as f32;
		let viewport_height = 2_f32;
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
				let pixel_center = self.p00_loc + (i as f32 * self.pxd_u) + (j as f32 * self.pxd_v);
				let ray_dir = pixel_center - self.center;

				let ray = Ray::new(self.center, ray_dir);

				let color = Self::ray_color(&ray, world);
				color::write_color(color);
			}
		}

		eprintln!("\rDone.                     ");
	}
}
