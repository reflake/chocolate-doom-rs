use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::{fixed::{FRACBITS, fixed}, tri_tables::{FINE_SINE, TAN_TO_ANGLE}, vector::concrete::vec2};

// Binary Angle Measument, BAM.

const DEG_360: i64 = 0x1_0000_0000;

#[repr(C)]
#[derive(Clone, Copy, PartialOrd, PartialEq, Eq)]
pub struct ang(pub u32);

impl ang {
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
		fixed(FINE_SINE[self.to_fine_shift() + 0x800])
	}
	pub const fn fine_sine(self) -> fixed {
		fixed(FINE_SINE[self.to_fine_shift()])
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
			let ans: usize = ((num.0 << 3) as isize / (den.0 >> 8) as isize).cast_unsigned();

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

impl Neg for ang {
	type Output = Self;

	fn neg(self) -> Self::Output {
		ang(self.0.wrapping_neg())
	}
}

#[allow(nonstandard_style)]
pub fn R_PointToAngle2(p1: vec2, p2: vec2)-> ang
{	
    return R_PointToAngle(p1 - p2);
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
                return ang(ang::degree(90.0).0 - 1) - ang::slope_div(point.x, point.y);  // octant 1
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
                return ang(ang::degree(180.0).0 - 1) - ang::slope_div(point.y, -point.x); // octant 3
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
                return ang(ang::degree(270.0).0 - 1) - ang::slope_div(-point.x, -point.y); // octant 5
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
			write!(f, "ang({:#X})", self.0)
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
}