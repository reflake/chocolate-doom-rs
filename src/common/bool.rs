use std::ops::Not;

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum bool32 {
	False = 0,
	True = 1,
}

pub const FALSE: bool32 = bool32::False;
pub const TRUE: bool32 = bool32::True;

impl From<bool32> for bool {
	fn from(b: bool32) -> bool {
		matches!(b, bool32::True)
	}
}

impl Not for bool32 {
	type Output = Self;

	fn not(self) -> Self::Output {
		match self {
			bool32::False => bool32::True,
			bool32::True => bool32::False,
		}
	}
}