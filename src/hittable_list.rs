use crate::{
	hittable::{HitRecord, Hittable},
	interval::Interval,
	ray::Ray,
};

pub struct HittableList {
	objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
	pub fn new() -> Self {
		Self {
			objects: vec![],
		}
	}

	pub fn clear(&mut self) {
		self.objects.clear();
	}

	pub fn add(&mut self, object: impl Hittable + 'static) {
		self.objects
			.push(Box::new(object));
	}
}

impl Hittable for HittableList {
	fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
		let mut temp_rec = HitRecord::new();
		let mut hit_anything = false;
		let mut closest_so_far = ray_t.max;

		for object in &self.objects {
			if object.hit(ray, Interval::new(ray_t.min, closest_so_far), rec) {
				hit_anything = true;
				closest_so_far = rec.t;
				temp_rec.set(rec);
			}
		}

		hit_anything
	}
}
