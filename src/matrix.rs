use Vector;

/// A 4D Matrix for calculating with 3D Vectors
#[derive(Copy, Clone, Debug, Default)]
pub struct Matrix {
	/// The internal data of the Matrix
	pub data: [[f32; 4]; 4],
}

impl Matrix {
	/// Creates a new Matrix with all entries set to 0
	///
	/// ```text
	/// [0  0  0  0]
	/// [0  0  0  0]
	/// [0  0  0  0]
	/// [0  0  0  0]
	/// ```
	pub fn new() -> Matrix {
		Default::default()
	}
	/// Creates an Identity Matrix with the diagonal set to 1 and everything else set to 0
	///
	/// ```text
	/// [1  0  0  0]
	/// [0  1  0  0]
	/// [0  0  1  0]
	/// [0  0  0  1]
	/// ```
	pub fn identity() -> Matrix {
		Matrix {
			data: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0],
			],
		}
	}
	/// Creates a LookAt Matrix for a Camera at `position` facing `looking_at` with up Vector `up`
	pub fn look_at(position: Vector, looking_at: Vector, up: Vector) -> Matrix {
		Matrix::view(position, looking_at - position, up)
	}
	/// Creates a View Matrix for a Camera at `position` facing in `direction` with up Vector `up`
	pub fn view(position: Vector, direction: Vector, up: Vector) -> Matrix {
		let f = direction.norm();

		let s = up.cross(f).norm();

		let u = f.cross(s).norm();

		let p = Vector {
			x: -position * s,
			y: -position * u,
			z: -position * f,
		};

		Matrix {
			data: [
				[s.x, s.x, s.x, p.x],
				[u.y, u.y, u.y, p.y],
				[f.z, f.z, f.z, p.z],
				[0.0, 0.0, 0.0, 1.0],
			],
		}
	}
	/// Creates a Projection Matrix for a ViewPort with dimensions `(width, height)`, a Field of View `fov` in Radians and the `near` and `far` Boundaries
	pub fn projection((width, height): (usize, usize), fov: f32, near: f32, far: f32) -> Matrix {
		let aspect_ratio = height as f32 / width as f32;

		let f = 1.0 / (fov / 2.0).tan();

		let dz = -(2.0 * far * near) / (far - near);

		Matrix {
			data: [
				[f * aspect_ratio, 0.0, 0.0, 0.0],
				[0.0, f, 0.0, 0.0],
				[0.0, 0.0, (far + near) / (far - near), dz],
				[0.0, 0.0, 1.0, 0.0],
			],
		}
	}
	/// Creates a Frustum Matrix with the given Boundaries
	pub fn frustum(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Matrix {
		let (l, r, t, b, n, f) = (left, right, top, bottom, near, far);

		let rml = right - left;
		let tmb = top - bottom;
		let fmn = far - near;

		Matrix {
			data: [
				[2.0 * n / rml, 0.0, (r + l) / rml, 0.0],
				[0.0, 2.0 * n / tmb, (t + b) / tmb, 0.0],
				[0.0, 0.0, -(f + n) / fmn, -2.0 * f * n / fmn],
				[0.0, 0.0, -1.0, 0.0],
			],
		}
	}
	/// Returns a Matrix created from Transposing this Matrix
	///
	/// A Transposed Matrix is mirrored along the diagonal, so that rows and columns are swapped
	pub fn transposed(&self) -> Matrix {
		let mut mat = Matrix::new();
		mat.iter_mut().enumerate().for_each(|(x, m)| {
			self.iter().enumerate().for_each(|(y, s)| {
				m[y] = s[x];
			});
		});
		mat
	}
	/// Creates a Translation Matrix for a translation by `delta`
	pub fn translate(delta: Vector) -> Matrix {
		let mut mat = Matrix::identity();
		mat[0][3] = delta.x;
		mat[1][3] = delta.y;
		mat[2][3] = delta.z;
		mat
	}
	/// Creates a Rotation Matrix for rotating around the x Axis by `radians`
	pub fn rot_x(radians: f32) -> Matrix {
		let (s, c) = radians.sin_cos();
		Matrix {
			data: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, c, s, 0.0],
				[0.0, -s, c, 0.0],
				[0.0, 0.0, 0.0, 1.0],
			],
		}
	}
	/// Creates a Rotation Matrix for rotating around the y Axis by `radians`
	pub fn rot_y(radians: f32) -> Matrix {
		let (s, c) = radians.sin_cos();
		Matrix {
			data: [
				[c, 0.0, -s, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[s, 0.0, c, 0.0],
				[0.0, 0.0, 0.0, 1.0],
			],
		}
	}
	/// Creates a Rotation Matrix for rotating around the z Axis by `radians`
	pub fn rot_z(radians: f32) -> Matrix {
		let (s, c) = radians.sin_cos();
		Matrix {
			data: [
				[c, -s, 0.0, 0.0],
				[s, c, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0],
			],
		}
	}
}

use std::ops::*;

impl Mul for &'_ Matrix {
	type Output = Matrix;
	fn mul(self, rhs: Self) -> Matrix {
		let mut ret = Matrix::new();
		for y in 0..4 {
			for x in 0..4 {
				let mut sum = 0.0;
				for i in 0..4 {
					sum += self.data[y][i] * rhs.data[i][x];
				}
				ret.data[x][y] = sum;
			}
		}
		ret
	}
}

impl Mul<Vector> for &Matrix {
	type Output = Vector;
	fn mul(self, rhs: Vector) -> Vector {
		let mut out = [0.0; 4];
		let rhs = [rhs.x, rhs.y, rhs.z, 1.0];
		out.iter_mut().zip(self.iter()).for_each(|(out, lhs)| {
			lhs.iter().zip(rhs.iter()).for_each(|(l, r)| {
				*out += l * r;
			});
		});
		Vector::from(&out[0..3]) * (1.0 / out[3])
	}
}

impl Mul<f32> for &Matrix {
	type Output = Matrix;
	fn mul(self, rhs: f32) -> Matrix {
		let mut out = *self;
		out.iter_mut()
			.flat_map(|row| row.iter_mut())
			.for_each(|v| *v *= rhs);
		out
	}
}

impl<'a, 'b> Add<&'b Matrix> for &'a Matrix {
	type Output = Matrix;
	fn add(self, rhs: &'b Matrix) -> Matrix {
		let mut out = *self;
		for i in 0..4 {
			for j in 0..4 {
				out[i][j] += rhs[i][j];
			}
		}
		out
	}
}

impl<'a, 'b> Sub<&'b Matrix> for &'a Matrix {
	type Output = Matrix;
	fn sub(self, rhs: &'b Matrix) -> Matrix {
		let mut out = *self;
		for i in 0..4 {
			for j in 0..4 {
				out[i][j] -= rhs[i][j];
			}
		}
		out
	}
}

impl Index<usize> for Matrix {
	type Output = [f32; 4];
	fn index(&self, index: usize) -> &[f32; 4] {
		&self.data[index]
	}
}
impl IndexMut<usize> for Matrix {
	fn index_mut(&mut self, index: usize) -> &mut [f32; 4] {
		&mut self.data[index]
	}
}

impl Deref for Matrix {
	type Target = [[f32; 4]; 4];
	fn deref(&self) -> &[[f32; 4]; 4] {
		&self.data
	}
}
impl DerefMut for Matrix {
	fn deref_mut(&mut self) -> &mut [[f32; 4]; 4] {
		&mut self.data
	}
}

use std::fmt::{Display, Formatter, Result};

impl Display for Matrix {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(
			f,
			"{:?}\n{:?}\n{:?}\n{:?}",
			self.data[0], self.data[1], self.data[2], self.data[3],
		)
	}
}
