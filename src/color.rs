use glam::Vec3;

pub type Color = Vec3;

pub fn write_color(color: Color) {
	let r = color.x;
	let g = color.y;
	let b = color.z;

	let ir = (r * 255.999) as u32;
	let ig = (g * 255.999) as u32;
	let ib = (b * 255.999) as u32;

	println!("{} {} {}", ir, ig, ib);
}
