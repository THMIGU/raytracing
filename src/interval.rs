use crate::utils::INFINITY;

#[derive(Clone, Copy)]
pub struct Interval {
	pub min: f32,
	pub max: f32,
}

impl Interval {
	pub const EMPTY: Interval = Self {
		min: INFINITY,
		max: -INFINITY,
	};
	pub const UNIVERSE: Interval = Self {
		min: -INFINITY,
		max: INFINITY,
	};

	pub fn new(min: f32, max: f32) -> Self {
		Self {
			min,
			max,
		}
	}

	pub fn size(&self) -> f32 {
		self.max - self.min
	}

	pub fn contains(&self, x: f32) -> bool {
		self.min <= x && x <= self.max
	}

	pub fn surrounds(&self, x: f32) -> bool {
		self.min < x && x < self.max
	}

	pub fn clamp(&self, x: f32) -> f32 {
		if x > self.max {
			return self.max;
		} else if x < self.min {
			return self.min;
		}
		x
	}
}

impl Default for Interval {
	fn default() -> Self {
		Self {
			min: -INFINITY,
			max: INFINITY,
		}
	}
}
