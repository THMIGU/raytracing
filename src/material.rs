use glam::Vec3;

use crate::{
	color::Color,
	hittable::HitRecord,
	ray::Ray,
	utils::{near_zero, random_float, random_unit_vec3},
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
	fuzz: f32,
}

impl Metal {
	pub fn new(albedo: Color, fuzz: f32) -> Self {
		Self {
			albedo,
			fuzz,
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
		let mut reflected = ray.dir.reflect(rec.normal);
		reflected = reflected.normalize() + (self.fuzz * random_unit_vec3());

		scattered.origin = rec.p;
		scattered.dir = reflected;

		attenuation.x = self.albedo.x;
		attenuation.y = self.albedo.y;
		attenuation.z = self.albedo.z;

		scattered.dir.dot(rec.normal) > 0_f32
	}
}

pub struct Dielectric {
	refraction_idx: f32,
}

impl Dielectric {
	pub fn new(refraction_idx: f32) -> Self {
		Self {
			refraction_idx,
		}
	}

	fn reflectance(cosine: f32, ri: f32) -> f32 {
		let mut r0 = (1_f32 - ri) / (1_f32 + ri);
		r0 = r0.powi(2);
		r0 + (1_f32 - r0) * (1_f32 - cosine).powi(5)
	}
}

impl Material for Dielectric {
	fn scatter(
		&self,
		ray: &Ray,
		rec: &HitRecord,
		attenuation: &mut Color,
		scattered: &mut Ray,
	) -> bool {
		attenuation.x = 1_f32;
		attenuation.y = 1_f32;
		attenuation.z = 1_f32;

		let ri = if rec.front_face {
			1_f32 / self.refraction_idx
		} else {
			self.refraction_idx
		};

		let unit_dir = ray.dir.normalize();
		let refracted = unit_dir.refract(rec.normal, ri);
		let dir;

		let cos_theta = 1_f32.min((-unit_dir).dot(rec.normal));

		if refracted == Vec3::ZERO || Self::reflectance(cos_theta, ri) > random_float(None) {
			dir = unit_dir.reflect(rec.normal);
		} else {
			dir = refracted;
		}

		scattered.origin = rec.p;
		scattered.dir = dir;

		true
	}
}
