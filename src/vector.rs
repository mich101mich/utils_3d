/// A 3-Dimensional Vector with x, y, z Components as f32
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
	/// the x Component
	pub x: f32,
	/// the y Component
	pub y: f32,
	/// the z Component
	pub z: f32,
}

impl Vector {
	/// Creates a new Vector with x, y, z Components set to 0.0
	pub fn new() -> Vector {
		Default::default()
	}
	/// Returns a new Vector with the x Component set to `x`
	pub fn x(self, x: f32) -> Vector {
		Vector { x, ..self }
	}
	/// Returns a new Vector with the y Component set to `y`
	pub fn y(self, y: f32) -> Vector {
		Vector { y, ..self }
	}
	/// Returns a new Vector with the z Component set to `z`
	pub fn z(self, z: f32) -> Vector {
		Vector { z, ..self }
	}
	/// Returns the [cross product](https://en.wikipedia.org/wiki/Cross_product) of two Vectors
	///
	/// The cross product of two Vectors a and b is defined as:
	///
	/// ```text
	/// x = a.y * b.z - a.z * b.y
	/// y = a.z * b.x - a.x * b.z
	/// z = a.x * b.y - a.y * b.x
	/// ```
	pub fn cross(self, rhs: Vector) -> Vector {
		Vector {
			x: self.y * rhs.z - self.z * rhs.y,
			y: self.z * rhs.x - self.x * rhs.z,
			z: self.x * rhs.y - self.y * rhs.x,
		}
	}
	/// Calculates the length of the Vector
	///
	/// The length of a Vector is defined as
	/// ```text
	/// len = sqrt(x^2 + y^2 + z^2)
	/// ```
	///
	/// this is the same method as [len_sq](#method.len_sq), except that it calculates the square root of the Result
	pub fn length(self) -> f32 {
		self.length_sq().sqrt()
	}
	/// Calculates the squared length of the Vector
	///
	/// this is the same method as [len](#method.len), except that it does not calculate the square root of the Result, making it slightly faster
	pub fn length_sq(self) -> f32 {
		self * self
	}
	/// Returns a normalized Vector pointing in the same direction as `self`
	///
	/// A normalized Vector has a length of exactly 1.
	/// ```
	/// # extern crate utils_3d; use utils_3d::Vector;
	/// let v = Vector { x: 5.0, y: 1.0, z: -3.5 };
	/// assert!((v.norm().length() - 1.0).abs() <= std::f32::EPSILON);
	/// ```
	pub fn norm(self) -> Vector {
		self / self.length()
	}
	/// Calculates the angle between two Vectors in Radians
	///
	/// ```
	/// # use utils_3d::Vector; use ::std::f32::consts::PI;
	/// let a = Vector { x: 1.0, y: 0.0, z: 0.0 };
	/// let b = Vector { x: 0.0, y: 1.0, z: 0.0 };
	/// assert!((a.angle(b) - PI / 2.0).abs() <= std::f32::EPSILON);
	/// ```
	/// This method always returns the smallest angle
	/// ```
	/// # use utils_3d::Vector;
	/// # let a = Vector { x: 1.0, y: 0.0, z: 0.0 };
	/// # let b = Vector { x: 0.0, y: 1.0, z: 0.0 };
	/// assert_eq!(a.angle(b), b.angle(a));
	/// ```
	pub fn angle(self, other: Vector) -> f32 {
		(self * other / (self.length() * other.length())).acos()
	}
}

impl From<[f32; 3]> for Vector {
	fn from(src: [f32; 3]) -> Vector {
		Vector {
			x: src[0],
			y: src[1],
			z: src[2],
		}
	}
}
impl From<&[f32]> for Vector {
	fn from(src: &[f32]) -> Vector {
		assert_eq!(
			src.len(),
			3,
			"Vector::from(&[f32]) Input slice has incorrect length: {} given, 3 expected",
			src.len()
		);
		Vector {
			x: src[0],
			y: src[1],
			z: src[2],
		}
	}
}
impl From<(f32, f32, f32)> for Vector {
	fn from(src: (f32, f32, f32)) -> Vector {
		Vector {
			x: src.0,
			y: src.1,
			z: src.2,
		}
	}
}

use std::ops::*;

impl Add for Vector {
	type Output = Vector;
	fn add(self, rhs: Vector) -> Vector {
		Vector {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}
impl AddAssign for Vector {
	fn add_assign(&mut self, rhs: Vector) {
		*self = *self + rhs;
	}
}
impl Sub for Vector {
	type Output = Vector;
	fn sub(self, rhs: Vector) -> Vector {
		Vector {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}
impl SubAssign for Vector {
	fn sub_assign(&mut self, rhs: Vector) {
		*self = *self - rhs;
	}
}
impl Mul for Vector {
	type Output = f32;
	fn mul(self, rhs: Vector) -> f32 {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}
}

impl Mul<f32> for Vector {
	type Output = Vector;
	fn mul(self, rhs: f32) -> Vector {
		Vector {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}
impl MulAssign<f32> for Vector {
	fn mul_assign(&mut self, rhs: f32) {
		*self = *self * rhs;
	}
}

impl Div<f32> for Vector {
	type Output = Vector;
	fn div(self, rhs: f32) -> Vector {
		Vector {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
		}
	}
}
impl DivAssign<f32> for Vector {
	fn div_assign(&mut self, rhs: f32) {
		*self = *self / rhs;
	}
}

use super::Matrix;

impl Mul<Matrix> for Vector {
	type Output = Vector;
	fn mul(self, rhs: Matrix) -> Vector {
		rhs * self
	}
}
impl MulAssign<Matrix> for Vector {
	fn mul_assign(&mut self, rhs: Matrix) {
		*self = rhs * *self;
	}
}

impl Neg for Vector {
	type Output = Vector;
	fn neg(self) -> Vector {
		Vector {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

impl PartialEq for Vector {
	fn eq(&self, rhs: &Vector) -> bool {
		use std::f32::EPSILON as epsilon;
		(self.x - rhs.x).abs() <= epsilon
			&& (self.y - rhs.y).abs() <= epsilon
			&& (self.z - rhs.z).abs() <= epsilon
	}
}

impl Index<usize> for Vector {
	type Output = f32;
	fn index(&self, index: usize) -> &f32 {
		match index {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			_ => panic!("Vector index out of Range: {} given, max 2", index),
		}
	}
}
impl IndexMut<usize> for Vector {
	fn index_mut(&mut self, index: usize) -> &mut f32 {
		match index {
			0 => &mut self.x,
			1 => &mut self.y,
			2 => &mut self.z,
			_ => panic!("Vector index out of Range: {} given, max 2", index),
		}
	}
}

use std::fmt::{Display, Formatter, Result};

impl Display for Vector {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z,)
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn vector_new() {
		let v: Vector = Vector::new();
		assert!((v.x - 0.0).abs() <= std::f32::EPSILON);
		assert!((v.y - 0.0).abs() <= std::f32::EPSILON);
		assert!((v.z - 0.0).abs() <= std::f32::EPSILON);
	}

}
