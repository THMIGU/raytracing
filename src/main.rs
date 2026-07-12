const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

fn main() {
	println!("P3\n{} {}\n255", WIDTH, HEIGHT);
	for j in 0..HEIGHT {
		eprint!("\rScanlines remaining: {} \r", HEIGHT - j);

		for i in 0..WIDTH {
			let r = i as f32 / (WIDTH - 1) as f32;
			let g = j as f32 / (HEIGHT - 1) as f32;
			let b = 0_f32;

			let ir = (r * 255.999) as u32;
			let ig = (g * 255.999) as u32;
			let ib = (b * 255.999) as u32;

			println!("{} {} {}", ir, ig, ib);
		}
	}

	eprintln!("\rDone.                     ");
}
