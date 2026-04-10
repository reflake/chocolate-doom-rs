use std::ops::{Div, Mul};

use common::fixed::fixed;

use crate::mobj::{Flags, Mobj};
use crate::map::utils::AproxDistance;

#[repr(C)]
pub struct Skull<'a> {
	pub mobj: &'a mut Mobj,
}

impl Skull<'_> {

	//
	// SkullAttack
	// Fly at the player like a missile.
	//
	pub fn attack(&mut self) {

		unsafe {
			let Some(dest) = self.mobj.target.as_ref() else {
				return;
			};

			self.mobj.flags.insert(Flags::SKULLFLY);

			self.mobj.emit_sound(self.mobj.info.as_ref().unwrap().attacksound);
			self.mobj.face_target();

			const SKULL_SPEED: fixed = fixed::from_int(20);

			let f_dist = self.mobj.position.xy().aprox_distance_to(dest.position.xy());
			let i_dist = f_dist.div(SKULL_SPEED).div(0x10000).max(fixed::EPSILON);
			let vel_z = (dest.position.z + dest.height / 2 - self.mobj.position.z) / i_dist.0;

			self.mobj.momentum = self.mobj.forward_xy()
										  .mul(SKULL_SPEED)
										  .with_z(vel_z);
		}
	}
}

#[unsafe(no_mangle)]
extern "C" fn A_SkullAttack(mut skull: Skull) {
	skull.attack();
}