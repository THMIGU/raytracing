use std::ops::Range;

use glam::{Vec3, vec3};

pub const INFINITY: f32 = f32::INFINITY;
const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
	degrees * PI / 180_f32
}

pub fn random_float(range: Option<Range<f32>>) -> f32 {
	rand::random_range(range.unwrap_or(0_f32..1_f32))
}

fn random_vec3(range: Option<Range<f32>>) -> Vec3 {
	vec3(random_float(range.clone()), random_float(range.clone()), random_float(range.clone()))
}

pub fn random_unit_vec3() -> Vec3 {
	loop {
		let vec = random_vec3(Some(-1_f32..1_f32));
		let length_sq = vec.length().powi(2);

		if length_sq <= 1_f32 && length_sq > f32::MIN {
			return vec.normalize();
		}
	}
}

pub fn random_vec3_hemisphere(normal: Vec3) -> Vec3 {
	let vec = random_unit_vec3();
	if normal.dot(vec) >= 0_f32 {
		return vec;
	}

	-vec
}

pub fn near_zero(v: Vec3) -> bool {
	let s = 1e-8;
	v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}
