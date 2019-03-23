use ray_tracing::HitInfo;
use vector::Vector;

/// A Ray in 3D-Space
#[derive(Clone, Copy, Debug)]
pub struct Ray {
	/// The starting Point of the Ray
	pub start: Vector,
	/// The direction where the Ray is headed
	///
	/// should be normalized, but is not guaranteed to be
	pub direction: Vector,
}

impl Ray {
	/// creates a new Ray with the given starting Point and direction
	///
	/// automatically normalizes the direction
	pub fn new(start: Vector, direction: Vector) -> Ray {
		Ray {
			start,
			direction: direction.norm(),
		}
	}
	/// Returns a Ray that is the result of reflecting this Ray at the hit Point
	pub fn reflect(&self, hit: &HitInfo) -> Ray {
		let dir = self.direction - hit.normal * 2.0 * (hit.normal * self.direction);
		Ray::new(hit.point, dir)
	}
}
