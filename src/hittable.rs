use glam::Vec3;

use crate::ray::Ray;

pub struct HitRecord {
	pub p: Vec3,
	pub normal: Vec3,
	pub t: f32,
	pub front_face: bool,
}

impl HitRecord {
	pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
		self.front_face = ray.dir.dot(outward_normal) < 0_f32;
		self.normal = if self.front_face {
			outward_normal
		} else {
			-outward_normal
		};
	}
}

pub trait Hittable {
	fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
