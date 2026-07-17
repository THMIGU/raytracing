use glam::Vec3Swizzles;

use crate::{
	color::Color,
	hittable::HitRecord,
	ray::Ray,
	utils::{near_zero, random_unit_vec3},
};

pub trait Material {
	fn scatter(
		&self,
		ray: &Ray,
		rec: &HitRecord,
		attenuation: &mut Color,
		scattered: &mut Ray,
	) -> bool;
}

pub struct Lambertian {
	albedo: Color,
}

impl Lambertian {
	pub fn new(albedo: Color) -> Self {
		Self {
			albedo,
		}
	}
}

impl Material for Lambertian {
	fn scatter(
		&self,
		ray: &Ray,
		rec: &HitRecord,
		attenuation: &mut Color,
		scattered: &mut Ray,
	) -> bool {
		let mut scatter_dir = rec.normal + random_unit_vec3();

		if near_zero(scatter_dir) {
			scatter_dir = rec.normal;
		}

		scattered.origin = rec.p;
		scattered.dir = scatter_dir;

		attenuation.x = self.albedo.x;
		attenuation.y = self.albedo.y;
		attenuation.z = self.albedo.z;

		true
	}
}

pub struct Metal {
	albedo: Color,
}

impl Metal {
	pub fn new(albedo: Color) -> Self {
		Self {
			albedo,
		}
	}
}

impl Material for Metal {
	fn scatter(
		&self,
		ray: &Ray,
		rec: &HitRecord,
		attenuation: &mut Color,
		scattered: &mut Ray,
	) -> bool {
		let reflected = ray.dir.reflect(rec.normal);
		scattered.origin = rec.p;
		scattered.dir = reflected;

		attenuation.x = self.albedo.x;
		attenuation.y = self.albedo.y;
		attenuation.z = self.albedo.z;

		true
	}
}
