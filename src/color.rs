use glam::Vec3;

use crate::interval::Interval;

pub type Color = Vec3;

pub fn write_color(color: Color) {
	let r = color.x;
	let g = color.y;
	let b = color.z;

	let intensity = Interval::new(0_f32, 0.999);

	let ir = (intensity.clamp(r) * 256_f32) as u32;
	let ig = (intensity.clamp(g) * 256_f32) as u32;
	let ib = (intensity.clamp(b) * 256_f32) as u32;

	println!("{} {} {}", ir, ig, ib);
}
