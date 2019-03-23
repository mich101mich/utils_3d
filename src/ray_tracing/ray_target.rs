use ray_tracing::Ray;
use vector::Vector;

/// A struct to store info about a Raycasting hit
///
/// The only necessary information that should be provided by any RayTarget is the point
/// and normal of the hit. Any additional information may be useful to the caster of the Ray,
/// but is not necessarily provided by all Implementations of RayTarget.
#[derive(Default)]
pub struct HitInfo {
	/// The Point where the Ray hit
	pub point: Vector,
	/// The Normal of the Object at the hit
	///
	/// This may be used to calculate a reflected Ray
	pub normal: Vector,
	/// The Color of the Object at the hit (_optional_)
	pub color: Option<u32>,
	/// The "reflectiveness" of the Object (_optional_)
	pub reflect_factor: Option<f32>,
}

/// A Trait for handling Raycasting on an Object
pub trait RayTarget {
	/// get the full info of a Ray hit
	/// 
	/// returns None if the Ray does not hit the Object
	fn hit_info(&self, ray: &Ray) -> Option<HitInfo>;
	/// get the Point where a Ray hits
	/// 
	/// returns None if the Ray does not hit the Object
	fn hit_point(&self, ray: &Ray) -> Option<Vector> {
		self.hit_info(ray).map(|info| info.point)
	}
	/// test if a Ray hits
	fn hits(&self, ray: &Ray) -> bool {
		self.hit_point(ray).is_some()
	}
}
