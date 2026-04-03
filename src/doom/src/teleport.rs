use std::ops::{Add, Not};

use bool_ext::BoolExt;
use common::{fixed::fixed, mode::{GameVersion, game_version}, ptr_as_ref, vector::concrete::{vec2, vec3}};

use crate::{mobj::{Flags, Mobj, MobjType}, sounds::{SfxEnum}};

unsafe extern "C" {
	pub fn P_TeleportByLineTag(line: *mut std::ffi::c_void) -> *mut Mobj;

	pub fn P_TeleportMove(thing: *mut Mobj, x: fixed, y: fixed) -> bool;
}

fn teleport_by_line_tag<'a>(line: *mut std::ffi::c_void) -> Option<&'a Mobj> {
	unsafe {
		let ptr = P_TeleportByLineTag(line);
		ptr_as_ref(ptr)
	}
}

pub enum TeleportError {
	CannotTeleportMissile,
	OtherSide,
	NoCorrespondingLine,
	CannotMove,
}

impl Mobj {
	pub fn teleport_move(&mut self, pos: vec2) -> bool {
		unsafe {
			P_TeleportMove(self, pos.x, pos.y)
		}
	}

	pub fn teleport(&mut self, line: *mut std::ffi::c_void, side: i32) -> Result<(), TeleportError> {

		type Error = TeleportError;

		// don't teleport missiles
		self.flags.contains(Flags::MISSILE).not()
				  .or_err(Error::CannotTeleportMissile)?;

		// Don't teleport if hit back of line,
		//  so you can get out of teleporter.
		(side != 1).or_err(Error::OtherSide)?;

		let dest = teleport_by_line_tag(line).ok_or(Error::NoCorrespondingLine)?;

		let old_pos = self.position;

		self.teleport_move(dest.position.xy())
			.or_err(Error::CannotMove)?;

		// The first Final Doom executable does not set self->z
		// when teleporting. This quirk is unique to this
		// particular version; the later version included in
		// some versions of the Id Anthology fixed this.
		if game_version() != GameVersion::exe_final {
			self.position.z = self.floorz;
		}

		self.player()
			.map(|player| player.viewz = self.position.z + player.viewheight);

		// spawn teleport fog at source and destination
		let fog = Mobj::spawn(old_pos, MobjType::MT_TFOG);

		fog.emit_sound(SfxEnum::sfx_telept);

		let spawn_pos = dest.position.xy().add(dest.forward_xy() * 20).with_z(self.position.z);
		let fog = Mobj::spawn(spawn_pos, MobjType::MT_TFOG);

		// emit sound, where?
		fog.emit_sound(SfxEnum::sfx_telept);

		// don't move for a bit
		self.player()
			.map(|_| self.reactiontime = 18);

		self.angle = dest.angle;
		self.momentum = vec3::ZERO;

		Ok(())
	}
}

//
// TELEPORTATION
//
#[unsafe(no_mangle)]
extern "C" fn EV_Teleport(
	line: *mut std::ffi::c_void, 
	side: i32, 
	thing: *mut Mobj) -> bool
{
	let thing = unsafe { &mut *thing };

	thing.teleport(line, side).is_ok()
}