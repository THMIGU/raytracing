pub const INFINITY: f32 = f32::INFINITY;
const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
	degrees * PI / 180_f32
}

pub fn random_float() -> f32 {
	rand::random_range(0_f32..1_f32)
}
