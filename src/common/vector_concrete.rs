use crate::fixed::fixed;
use crate::vector::{*};

macro_rules! impl_concrete_vec_ops {
    ($concrete:ty, $scalar:ty, $n:expr) => {

		impl $concrete {
			pub const fn as_gen(self) -> vec<$scalar, $n> {
				unsafe {
					std::mem::transmute(self)
				}
			}
		}

		impl vec<$scalar, $n> {
			pub const fn as_concrete(self) -> $concrete {
				unsafe {
					std::mem::transmute(self)
				}
			}
		}

        impl Add<vec<$scalar, $n>> for $concrete {
            type Output = $concrete;

            fn add(self, rhs: vec<$scalar, $n>) -> Self::Output {
                (self.as_gen() + rhs).as_concrete()
            }
        }

        impl Sub<vec<$scalar, $n>> for $concrete {
            type Output = $concrete;

            fn sub(self, rhs: vec<$scalar, $n>) -> Self::Output {
                (self.as_gen() - rhs).as_concrete()
            }
        }

        impl Add<$concrete> for vec<$scalar, $n> {
            type Output = $concrete;

            fn add(self, rhs: $concrete) -> Self::Output {
                (self + rhs.as_gen()).as_concrete()
            }
        }

        impl Sub<$concrete> for vec<$scalar, $n> {
            type Output = $concrete;

            fn sub(self, rhs: $concrete) -> Self::Output {
                (self - rhs.as_gen()).as_concrete()
            }
        }

        impl Add<$concrete> for $concrete {
            type Output = $concrete;

            fn add(self, rhs: $concrete) -> Self::Output {
                (self.as_gen() + rhs.as_gen()).as_concrete()
            }
        }

        impl Sub<$concrete> for $concrete {
            type Output = $concrete;

            fn sub(self, rhs: $concrete) -> Self::Output {
                (self.as_gen() - rhs.as_gen()).as_concrete()
            }
        }

		impl AddAssign<vec<$scalar, $n>> for $concrete {
			fn add_assign(&mut self, rhs: vec<$scalar, $n>) {
				let vec: &mut vec<$scalar, $n> = unsafe { std::mem::transmute(self) };

				*vec += rhs;
			}
		}

		impl SubAssign<vec<$scalar, $n>> for $concrete {
			fn sub_assign(&mut self, rhs: vec<$scalar, $n>) {
				let vec: &mut vec<$scalar, $n> = unsafe { std::mem::transmute(self) };

				*vec -= rhs;
			}
		}

		impl AddAssign<$concrete> for $concrete {
			fn add_assign(&mut self, rhs: $concrete) {
				let vec: &mut vec<$scalar, $n> = unsafe { std::mem::transmute(self) };

				*vec += rhs.as_gen();
			}
		}

		impl SubAssign<$concrete> for $concrete {
			fn sub_assign(&mut self, rhs: $concrete) {
				let vec: &mut vec<$scalar, $n> = unsafe { std::mem::transmute(self) };

				*vec -= rhs.as_gen();
			}
		}

		impl <R> Mul<R> for $concrete
			where R: Copy,
			 $scalar: Copy + Mul<R, Output = $scalar>
		{
			type Output = $concrete;

			fn mul(self, rhs: R) -> Self::Output {
				(self.as_gen() * rhs).as_concrete()
			}
		}
    };
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct vec2 {
	pub x: fixed, pub y: fixed
}

impl vec2 {

	pub const ZERO: vec2 = Self::new(fixed(0), fixed(0));

	pub const fn new(x: fixed, y: fixed) -> Self {
		Self { x, y }
	}

	pub fn from_xy(x: impl Into<fixed>, y: impl Into<fixed>) -> Self {
		Self { x: x.into(), y: y.into() }
	}

	pub fn with_x(self, x: impl Into<fixed>) -> vec2 {
		vec2::from_xy(x, self.y)
	}

	pub fn with_y(self, y: impl Into<fixed>) -> vec2 {
		vec2::from_xy(self.x, y)
	}

	pub fn with_z(self, z: impl Into<fixed>) -> vec3 {
		vec3::from_xyz(self.x, self.y, z)
	}
}

impl_concrete_vec_ops!(vec2, fixed, 2);

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct vec3 {
	pub x: fixed, pub y: fixed, pub z: fixed,
}

impl vec3 {
	pub const ZERO: vec3 = Self::new(fixed(0), fixed(0), fixed(0));

	pub const fn new(x: fixed, y: fixed, z: fixed) -> Self {
		Self { x, y, z }
	}

	pub fn from_xy(x: impl Into<fixed>, y: impl Into<fixed>) -> Self {
		Self { x: x.into(), y: y.into(), z: 0.into() }
	}

	pub fn from_xyz(x: impl Into<fixed>, y: impl Into<fixed>, z: impl Into<fixed>) -> Self {
		Self { x: x.into(), y: y.into(), z: z.into() }
	}

	pub fn xy(self) -> vec2 {
		vec2::new(self.x, self.y)
	}

	pub fn with_x(self, x: impl Into<fixed>) -> vec3 {
		vec3::from_xyz(x, self.y, self.z)
	}

	pub fn with_y(self, y: impl Into<fixed>) -> vec3 {
		vec3::from_xyz(self.x, y, self.z)
	}

	pub fn with_z(self, z: impl Into<fixed>) -> vec3 {
		vec3::from_xyz(self.x, self.y, z)
	}
}

impl_concrete_vec_ops!(vec3, fixed, 3);

pub mod debug {

	use std::fmt::Debug;
	use super::vec2;

	impl Debug for vec2 {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "({} {})", self.x.to_double(), self.y.to_double())
		}
	}
}

#[cfg(test)]
pub mod test {

	use super::*;
	use rstest::rstest;

	#[rstest]
	fn haha() {
		let mut a = vec2::from_xy(10, 10);
		let b = vec2::from_xy(5, 25);
		
		a += b;

		let c = a + b;

		println!("a {:?} b {:?} c {:?}", a, b, c)
	}
}