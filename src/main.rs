mod color;

use crate::color::Color;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {
	println!("P3\n{} {}\n255", WIDTH, HEIGHT);
	for j in 0..HEIGHT {
		eprint!("\rScanlines remaining: {} \r", HEIGHT - j);

		for i in 0..WIDTH {
			let r = i as f32 / (WIDTH - 1) as f32;
			let g = j as f32 / (HEIGHT - 1) as f32;

			let color = Color::new(r, g, 0_f32);
			color::write_color(color);
		}
	}

	eprintln!("\rDone.                     ");
}
