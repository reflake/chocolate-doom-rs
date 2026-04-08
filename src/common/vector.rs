#[path = "vector_concrete.rs"]
pub mod concrete;

use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct vec<T, const L: usize> {
	arr: [T; L]
}

impl <T: Copy + Add<T, Output = T>, const L: usize> Add for vec<T, L> {
	type Output = vec<T, L>;

	fn add(self, rhs: Self) -> Self::Output {
		let mut arr = self.arr;

		for i in 0..L {
			arr[i] = arr[i] + rhs.arr[i];
		}

		Self {
			arr: arr
		}
	}
}

impl <T: Copy + AddAssign<T>, const L: usize> AddAssign for vec<T, L> {
	fn add_assign(&mut self, rhs: Self) {
		for i in 0..L {
			self.arr[i] += rhs.arr[i];
		}
	}
}

impl <T: Copy + Sub<T, Output = T>, const L: usize> Sub for vec<T, L> {
	type Output = vec<T, L>;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut arr = self.arr;

		for i in 0..L {
			arr[i] = arr[i] - rhs.arr[i];
		}

		Self {
			arr: arr
		}
	}
}

impl <T: Copy + SubAssign<T>, const L: usize> SubAssign for vec<T, L> {
	fn sub_assign(&mut self, rhs: Self) {
		for i in 0..L {
			self.arr[i] -= rhs.arr[i];
		}
	}
}

impl <T, R, const L: usize> Mul<R> for vec<T, L>
	where T: Copy + Mul<R, Output = T>,
	      R: Copy
{
	type Output = Self;

	fn mul(self, rhs: R) -> Self::Output {
		let mut arr = self.arr;

		for i in 0..L {
			arr[i] = arr[i] * rhs;
		}

		Self {
			arr: arr
		}
	}
}

impl <T, R, const L: usize> Div<R> for vec<T, L>
	where T: Copy + Div<R, Output = T>,
	      R: Copy
{
	type Output = Self;

	fn div(self, rhs: R) -> Self::Output {
		let mut arr = self.arr;

		for i in 0..L {
			arr[i] = arr[i] / rhs;
		}

		Self {
			arr: arr
		}
	}
}

mod debug {
    use std::fmt::Debug;
	use super::vec;

	impl <T: Debug, const N: usize> Debug for vec<T, N> {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{:?}", self.arr)
		}
	}
}