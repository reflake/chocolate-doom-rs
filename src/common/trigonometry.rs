use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use crate::{fixed::{FRACBITS, fixed}, tri_tables::{FINE_SINE, TAN_TO_ANGLE}, vector::concrete::vec2};

// Binary Angle Measument, BAM.

const DEG_360: i64 = 0x1_0000_0000;

#[repr(C)]
#[derive(Clone, Copy, PartialOrd, PartialEq, Eq)]
pub struct ang(pub u32);

impl ang {

	pub const ZERO: ang = ang(0);

	pub const fn degree(deg: f64) -> ang {

		let c = (deg / 360.0).fract() * DEG_360 as f64;

		if deg > 0.0 {
			ang(c as u32)
		} else {
			ang((-c as u32).wrapping_neg())
		}
	}
	pub const fn from_hi(hi_angle: i16) -> ang {
		ang((hi_angle as u32) << FRACBITS)
	}
	pub const fn fine_cosine(self) -> fixed {
		FINE_SINE[self.to_fine_shift() + 0x800]
	}
	pub const fn fine_sine(self) -> fixed {
		FINE_SINE[self.to_fine_shift()]
	}

	// to get a global angle from cartesian coordinates, the coordinates are
	// flipped until they are in the first octant of the coordinate system, then
	// the y (<=x) is scaled and divided by x to get a tangent (slope) value
	// which is looked up in the tantoangle[] table.  The +1 size is to handle
	// the case when x==y without additional checking.
	pub const fn slope_div(num: fixed, den: fixed) -> ang {
		const SLOPE_RANGE: usize = 2048;

		if den.0 < 512
		{
			TAN_TO_ANGLE[SLOPE_RANGE]
		}
		else
		{
			let ans: usize = (((num.0 << 3) as isize).wrapping_div((den.0 >> 8) as isize)).cast_unsigned();

			if ans <= SLOPE_RANGE
			{
				TAN_TO_ANGLE[ans]
			}
			else
			{
				TAN_TO_ANGLE[SLOPE_RANGE]
			}
		}
	}
	const fn to_fine_shift(self) -> usize {
		(self.0 >> 19) as usize
	}

	pub const fn to_degree(self) -> f64 {
		if self.is_neg() {
			(self.0 as f64 / DEG_360 as f64) * 360.0 - 360.0
		} else {
			(self.0 as f64 / DEG_360 as f64) * 360.0
		}
	}

	pub const fn is_neg(self) -> bool {
		self.0 > 0x8000_0000
	}
}

impl Add for ang {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		ang(self.0.wrapping_add(rhs.0))
	}
}

impl AddAssign for ang {
	fn add_assign(&mut self, rhs: ang) {
		self.0 = self.0.wrapping_add(rhs.0)
	}
}

impl Sub for ang {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		ang(self.0.wrapping_sub(rhs.0))
	}
}

impl SubAssign for ang {
	fn sub_assign(&mut self, rhs: ang) {
		self.0 = self.0.wrapping_sub(rhs.0)
	}
}

impl Mul<i32> for ang {
	type Output = Self;

	fn mul(self, rhs: i32) -> Self::Output {
		ang(self.0.wrapping_mul(rhs as u32))
	}
}

impl Div<i32> for ang {
	type Output = Self;

	fn div(self, rhs: i32) -> Self::Output {
		if self.is_neg() {
			ang(self.neg().0.wrapping_div(rhs as u32).wrapping_neg())
		} else {
			ang(self.0.wrapping_div(rhs as u32))
		}
	}
}

impl Neg for ang {
	type Output = Self;

	fn neg(self) -> Self::Output {
		ang(self.0.wrapping_neg())
	}
}

#[allow(nonstandard_style)]
pub fn R_PointToAngle2(p1: vec2, p2: vec2)-> ang
{	
    return R_PointToAngle(p2 - p1);
}

#[allow(nonstandard_style)]
pub fn R_PointToAngle(point: vec2) -> ang
{
    if point == vec2::ZERO {
        return ang::degree(0.0);
	}

    if point.x >= 0.into()
    {                           // x >=0
        if point.y >= 0.into()
        {                       // y>= 0
            if point.x > point.y
			{
                return ang::slope_div(point.y, point.x);      // octant 0
			}
            else {
                return ang(ang::degree(90.0).0.wrapping_sub(1)) - ang::slope_div(point.x, point.y);  // octant 1
			}
        }
        else
        {                       // y<0
            if point.x > -point.y
			{
                return -ang::slope_div(-point.y, point.x);     // octant 8
			}
            else
			{
                return ang::degree(270.0) + ang::slope_div(point.x, -point.y);     // octant 7
			}
        }
    }
    else
    {                           // x<0
        if point.y >= 0.into()
        {                       // y>= 0
            if -point.x > point.y
			{
                return ang(ang::degree(180.0).0.wrapping_sub(1)) - ang::slope_div(point.y, -point.x); // octant 3
			}
            else 
			{
                return ang(ang::degree(90.0).0) + ang::slope_div(-point.x, point.y);      // octant 2
			}
        }
        else
        {                       // y<0
            if -point.x > -point.y
			{
                return ang::degree(180.0) + ang::slope_div(-point.y, -point.x);     // octant 4
			}
            else 
			{
                return ang(ang::degree(270.0).0.wrapping_sub(1)) - ang::slope_div(-point.x, -point.y); // octant 5
			}
        }
    }
}

#[cfg(test)]
mod angle_tests {
	use std::fmt::Debug;

	use rstest::rstest;

	use super::*;

	impl Debug for ang {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "({}°: {:#X})", self.to_degree(), self.0)
		}
	}

	#[rstest]
	#[case(0.0, 0)]
	#[case(5.0, 0x038E_38E3)]
	#[case(-5.0, 0xFC71_C71D)]
	#[case(45.0, 0x2000_0000)]
	#[case(90.0, 0x4000_0000)]
	#[case(180.0, 0x8000_0000)]
	#[case(270.0, 0xC000_0000)]
	#[case(360.0, 0)]
	fn bit_angle_by_degrees(#[case] deg: f64, #[case] expected: u32) {
		assert_eq!(ang::degree(deg).0, expected);
	}

	#[rstest]
	#[case(270.0, 180.0, 90.0)]
	#[case(180.0, 270.0, 90.0)]
	#[case(45.0, 360.0, 45.0)]
	fn addition_of_angles_wraps_around(#[case] deg1: f64, #[case] deg2: f64, #[case] expected: f64) {
		let a1 = ang::degree(deg1);
		let a2 = ang::degree(deg2);
		assert_eq!(a1 + a2, ang::degree(expected));
	}

	#[rstest]
	#[case(0x4000u16, 90.0)]
	#[case(0xC000u16, 270.0)]
	fn from_hi_angle(#[case] hi_angle: u16, #[case] expected: f64) {
		assert_eq!(ang::from_hi(hi_angle as i16).0, ang::degree(expected).0);
	}

	#[rstest]
	#[case(5.0)]
	fn negation_of_angle(#[case] deg: f64) {
		let a = ang::degree(deg);
		assert_eq!(-a, ang::degree(-deg));
	}

	#[rstest]
	#[case(10.0, 2, 20.0)]
	#[case(5.0, 2, 9.999999906867743)]
	#[case(-20.0, 3, -60.0)]
	fn multiplication_of_angle(#[case] deg: f64, #[case] multiplier: i32, #[case] expected: f64) {
		let a = ang::degree(deg);
		assert_eq!(a * multiplier, ang::degree(expected));
	}

	#[rstest]
	#[case(30.0, 4, 7.5)]
	#[case(-90.0, 2, -45.0)]
	#[case(360.0, 8, 0.0)]
	fn division_of_angle(#[case] deg: f64, #[case] divisor: i32, #[case] expected: f64) {
		let a = ang::degree(deg);
		assert_eq!(a / divisor, ang::degree(expected));
	}
}