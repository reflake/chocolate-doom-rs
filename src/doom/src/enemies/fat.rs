use common::trigonometry::ang;

use crate::{external::INTERFACE, mobj::{Mobj, MobjType}, sounds::SfxEnum};

#[repr(C)]
pub struct Fat<'a> {
	pub mobj: &'a mut Mobj,
}

impl Fat<'_> {
	pub fn raise(&mut self) {
		self.mobj.face_target();
		self.mobj.emit_sound(SfxEnum::sfx_manatk);
	}
	//
	// Mancubus attack,
	// firing three missiles (bruisers)
	// in three different directions?
	// Doesn't look like it. 
	//
	const FATSPREAD: ang = ang::degree(90.0 / 8.0);

	pub fn attack(&mut self, attack: i32)
	{
		self.mobj.face_target();

		let target = Mobj::P_SubstNullMobj(self.mobj.target);

		match attack {
			0 => {
				// Change direction  to ...
				self.mobj.angle += Self::FATSPREAD;
				self.spawn_missile(target, ang::ZERO);
				self.spawn_missile(target, Self::FATSPREAD);
			},
			1 => {
    			// Now here choose opposite deviation.
				self.mobj.angle -= Self::FATSPREAD;
				self.spawn_missile(target, ang::ZERO);
				self.spawn_missile(target, -Self::FATSPREAD * 2);
			},
			2 => {
				self.spawn_missile(target, -Self::FATSPREAD / 2);
				self.spawn_missile(target, Self::FATSPREAD / 2);
			}
			_ => panic!("Invalid attack type for Fat: {}", attack),
		}
	}

	fn spawn_missile(&self, target: &mut Mobj, spread: ang)
	{
		let missile = self.mobj.spawn_missile(target, MobjType::MT_FATSHOT).unwrap();

		if spread != ang::ZERO {

			let speed = unsafe { missile.info.as_ref() }.unwrap().speed;

			missile.angle += spread;
			*missile.momentum.xy_mut() = missile.forward_xy() * speed;
		}
	}
}

#[unsafe(no_mangle)]
extern "C" fn A_FatRaise(mut fat: Fat) {
	fat.raise();
}

#[unsafe(no_mangle)]
extern "C" fn A_FatAttack1(mut fat: Fat) {
	fat.attack(0);
}

#[unsafe(no_mangle)]
extern "C" fn A_FatAttack2(mut fat: Fat) {
	fat.attack(1);
}

#[unsafe(no_mangle)]
extern "C" fn A_FatAttack3(mut fat: Fat) {
	fat.attack(2);
}