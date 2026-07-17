use std::rc::Rc;

use glam::Vec3;

use crate::{
	color::Color,
	interval::Interval,
	material::{Lambertian, Material},
	ray::Ray,
};

pub struct HitRecord {
	pub p: Vec3,
	pub normal: Vec3,
	pub mat: Rc<dyn Material>,
	pub t: f32,
	pub front_face: bool,
}

impl HitRecord {
	pub fn new() -> Self {
		Self {
			p: Vec3::ZERO,
			normal: Vec3::ZERO,
			mat: Rc::new(Lambertian::new(Color::ONE)),
			t: 0_f32,
			front_face: false,
		}
	}

	pub fn set(&mut self, rec: &HitRecord) {
		self.p = rec.p;
		self.normal = rec.normal;
		self.mat = rec.mat.clone();
		self.t = rec.t;
		self.front_face = rec.front_face;
	}

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
	fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
