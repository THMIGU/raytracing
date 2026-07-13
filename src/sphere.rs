use glam::Vec3;

use crate::{
	hittable::{HitRecord, Hittable},
	interval::Interval,
	ray::Ray,
};

pub struct Sphere {
	center: Vec3,
	radius: f32,
}

impl Sphere {
	pub fn new(center: Vec3, radius: f32) -> Self {
		Self {
			center,
			radius,
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
		let oc = self.center - ray.origin;

		let a = ray.dir.dot(ray.dir);
		let h = ray.dir.dot(oc);
		let c = oc.dot(oc) - self.radius.powi(2);

		let discriminant: f32 = h.powi(2) - a * c;
		if discriminant < 0_f32 {
			return false;
		}

		let sqrt_d = discriminant.sqrt();

		let mut root = (h - sqrt_d) / a;
		if !ray_t.surrounds(root) {
			root = (h + sqrt_d) / a;
			if !ray_t.surrounds(root) {
				return false;
			}
		}

		rec.t = root;
		rec.p = ray.at(root);
		let outward_normal = (rec.p - self.center) / self.radius;
		rec.set_face_normal(ray, outward_normal);

		true
	}
}
