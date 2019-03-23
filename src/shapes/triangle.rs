use vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
	pub corners: [Vector; 3],
}

impl Triangle {
	/// creates a new Triangle with the given corners
	pub fn new(a: Vector, b: Vector, c: Vector) -> Triangle {
		Triangle { corners: [a, b, c] }
	}
	/// calculates the area of the Triangle
	pub fn area(&self) -> f32 {
		// area of Triangle is half the area of a Parallelogram with sides (ab, ac)
		// area of Parallelogram = length of cross product of sides
		0.5 * (self[1] - self[0]).cross(self[2] - self[0]).length()
	}
	/// calculates a normalized Vector that is perpendicular to the Plane of the Triangle
	pub fn normal(&self) -> Vector {
		(self[1] - self[0]).cross(self[2] - self[0]).norm()
	}
	/// checks if the point is within the Triangle
	///
	/// assumes that the Point is on the same Plane as the Triangle
	pub fn contains(&self, point: Vector) -> bool {
		let mut neg_count = 0;
		for a in 0..2 {
			for b in (a + 1)..3 {
				if (self[a] - point) * (self[b] - point) <= 0.0 {
					neg_count += 1;
				}
			}
		}
		neg_count >= 2
	}
}

use ray_tracing::*;

impl RayTarget for Triangle {
	fn hit_info(&self, ray: &Ray) -> Option<HitInfo> {
		let n = self.normal();
		let a = (self[0] - ray.start) * n;
		let b = ray.direction * n;
		if b == 0.0 {
			return None;
		}
		let point = ray.start + ray.direction * a / b;
		if self.contains(point) {
			Some(HitInfo {
				point,
				normal: n,
				..Default::default()
			})
		} else {
			None
		}
	}
}

use std::ops::*;

impl Index<usize> for Triangle {
	type Output = Vector;
	fn index(&self, index: usize) -> &Vector {
		&self.corners[index]
	}
}
impl IndexMut<usize> for Triangle {
	fn index_mut(&mut self, index: usize) -> &mut Vector {
		&mut self.corners[index]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn triangle_new() {
		let a = Vector::from((2.0, 1.0, 0.0));
		let b = Vector::from((1.0, 3.0, 2.0));
		let c = Vector::from((1.0, 1.0, 1.0));
		let t = Triangle::new(a, b, c);
		assert_eq!(t.corners[0], a);
		assert_eq!(t.corners[1], b);
		assert_eq!(t.corners[2], c);
	}

	#[test]
	fn triangle_index() {
		let a = Vector::from((2.0, 1.0, 0.0));
		let b = Vector::from((1.0, 3.0, 2.0));
		let c = Vector::from((1.0, 1.0, 1.0));
		let t = Triangle::new(a, b, c);
		assert_eq!(t[0], a);
		assert_eq!(t[1], b);
		assert_eq!(t[2], c);
	}

	#[test]
	fn triangle_area() {
		let a = Vector::from((2.0, 1.0, 0.0));
		let b = Vector::from((1.0, 3.0, 2.0));
		let c = Vector::from((1.0, 1.0, 1.0));
		let t = Triangle::new(a, b, c);
		assert!((t.area() - 1.5).abs() <= std::f32::EPSILON);
	}

}
