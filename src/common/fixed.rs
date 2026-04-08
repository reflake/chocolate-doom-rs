
//
// DESCRIPTION:
//	Fixed point arithemtics, implementation.
//
use std::{i32, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

//
// Fixed point, 32bit as 16.16.
//
pub const FRACBITS: i32	= 16;
pub const FRACMASK: i32 = (2 << FRACBITS - 1) - 1;
pub const FRACUNIT: i32	= 1 << FRACBITS;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct fixed(pub i32);

impl Add for fixed {
	type Output = fixed;

	fn add(self, rhs: fixed) -> fixed {
		fixed(self.0.wrapping_add(rhs.0))
	}
}

impl AddAssign for fixed {
	fn add_assign(&mut self, rhs: fixed) {
		self.0 = self.0.wrapping_add(rhs.0)
	}
}

impl Sub for fixed {
	type Output = fixed;

	fn sub(self, rhs: fixed) -> fixed {
		fixed(self.0.wrapping_sub(rhs.0))
	}
}

impl SubAssign for fixed {
	fn sub_assign(&mut self, rhs: fixed) {
		self.0 = self.0.wrapping_sub(rhs.0)
	}
}

impl Mul for fixed {
	type Output = fixed;

	fn mul(self, rhs: fixed) -> fixed {
		fixed((self.0 as i64 * rhs.0 as i64 >> FRACBITS) as i32)
	}
}

impl Mul<i32> for fixed {
	type Output = fixed;

	fn mul(self, rhs: i32) -> Self::Output {
		fixed(self.0 * rhs)
	}
}

impl MulAssign for fixed {
	fn mul_assign(&mut self, rhs: fixed) {
		self.0 = self.mul(rhs).0
	}
}

impl MulAssign<i32> for fixed {
	fn mul_assign(&mut self, rhs: i32) {
		self.0 *= rhs
	}
}

impl Div for fixed {
	type Output = fixed;

	fn div(self, rhs: fixed) -> fixed {
		if self.0.abs() >> 14 >= rhs.0.abs() {
			return match self.0 ^ rhs.0 {
				xor if xor < 0 => fixed(i32::MIN),
				_              => fixed(i32::MAX)
			}
		}

		let result = ((self.0 as i64) << FRACBITS) / rhs.0 as i64;

		fixed(result as i32)
	}
}

impl DivAssign for fixed {
	fn div_assign(&mut self, rhs: Self) {
		self.0 = self.div(rhs).0
	}
}

impl Neg for fixed {
	type Output = fixed;

	fn neg(self) -> fixed {
		fixed(-self.0)
	}
}

// Conversion between fixed and floating representation
impl fixed {
	pub const fn to_double(self) -> f64 {
		self.0 as f64 / (1u64 << FRACBITS) as f64
	}
}

impl From<i32> for fixed {
	fn from(value: i32) -> Self {
		fixed(value << FRACBITS)
	}
}

impl From<f64> for fixed {
	fn from(value: f64) -> Self {
		fixed((value * (FRACMASK + 1) as f64) as i32)
	}
}

pub mod debug {
    use std::fmt::Debug;

    use crate::fixed::fixed;

	impl Debug for fixed {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{}", self.to_double())
		}
	}
}