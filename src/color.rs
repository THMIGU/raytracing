use glam::Vec3;

use crate::interval::Interval;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f32) -> f32 {
	if linear_component > 0_f32 {
		return linear_component.sqrt();
	}

	0_f32
}

pub fn write_color(color: Color) {
	let mut r = color.x;
	let mut g = color.y;
	let mut b = color.z;

	r = linear_to_gamma(r);
	g = linear_to_gamma(g);
	b = linear_to_gamma(b);

	let intensity = Interval::new(0_f32, 0.999);

	let ir = (intensity.clamp(r) * 256_f32) as u32;
	let ig = (intensity.clamp(g) * 256_f32) as u32;
	let ib = (intensity.clamp(b) * 256_f32) as u32;

	println!("{} {} {}", ir, ig, ib);
}
