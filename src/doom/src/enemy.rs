#[path = "enemies/mod.rs"]
mod enemies;

use common::trigonometry::{R_PointToAngle2, ang};

use crate::{mobj::*, random};

impl Mobj
{
	#[allow(static_mut_refs)]
	pub fn face_target(&mut self) {
		if let Some(target) = unsafe { self.target.as_ref() } {
			
			self.flags.remove(Flags::AMBUSH);

			self.angle = R_PointToAngle2(self.position.xy(), target.position.xy());

			// If the target is a partially invisible, offset angle by a random amount to make it harder to hit
			if target.flags.contains(Flags::SHADOW) {
				let spread = ang(unsafe { random::pred_random.next_sub().cast_unsigned() << 21 });
				self.angle += spread;
			}
		}
	}
}

#[unsafe(no_mangle)]
extern "C" fn A_FaceTarget(mobj: &mut Mobj) {
	mobj.face_target();
}