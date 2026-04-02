use std::ptr::{null, null_mut};

use common::{fixed::fixed, mode::{GetGameVersion, GameVersion}};

use crate::{mobj::{Flags, Mobj, MobjType, P_SpawnMobj}, sounds::{S_StartSound, SfxEnum}};

unsafe extern "C" {
	pub fn P_TeleportByLineTag(line: *mut std::ffi::c_void) -> *const std::ffi::c_void;

	pub fn P_TeleportMove(thing: *mut Mobj, x: fixed, y: fixed) -> bool;
}

//
// TELEPORTATION
//
#[unsafe(no_mangle)]
extern "C" fn EV_Teleport(
	line: *mut std::ffi::c_void, 
	side: i32, 
	p_thing: *mut Mobj) -> bool
{
	unsafe {
    	// don't teleport missiles
		let thing = &mut *p_thing;

		if thing.flags.contains(Flags::MISSILE) {
			return false;
		}

		// Don't teleport if hit back of line,
		//  so you can get out of teleporter.
		if side == 1 {
			return false;
		}

		let target = P_TeleportByLineTag(line);

		if target.is_null() {
			return false;
		}

		let target = target.cast::<Mobj>().as_ref().unwrap();

		let old_pos = (thing.x, thing.y, thing.z);

		if !P_TeleportMove(p_thing, target.x, target.y) {
			return false;
		}

		// The first Final Doom executable does not set thing->z
		// when teleporting. This quirk is unique to this
		// particular version; the later version included in
		// some versions of the Id Anthology fixed this.
		if GetGameVersion() != GameVersion::exe_final {
			thing.z = thing.floorz;
		}

		if !thing.player.is_null() {
			let player = &mut *thing.player;

			player.viewz = thing.z + player.viewheight;
		}

		// spawn teleport fog at source and destination
		let fog = P_SpawnMobj(old_pos.0, old_pos.1, old_pos.2, MobjType::MT_TFOG);

		S_StartSound(fog, SfxEnum::sfx_telept);

		let fog = P_SpawnMobj(target.x + target.angle.fine_cosine() * 20, 
							  target.y + target.angle.fine_sine() * 20,
							  thing.z, MobjType::MT_TFOG);

		// emit sound, where?
		S_StartSound(fog, SfxEnum::sfx_telept);

		// don't move for a bit
		if !thing.player.is_null() {
		    thing.reactiontime = 18;
		}

		thing.angle = target.angle;
		thing.momx = fixed(0);
		thing.momy = fixed(0);
		thing.momz = fixed(0);

		return true;
	}
}