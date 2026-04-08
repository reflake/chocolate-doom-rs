// Doom External hehe

use common::fixed::fixed;
use std::ffi::{CStr, CString, c_void};

use crate::{info::StateEnum, mobj::{Mobj, MobjType}, player::Player, sounds::SfxEnum};

// An interface to the external C library functions.
#[repr(C)]
#[allow(nonstandard_style)]
pub struct Interface {
	
	P_TeleportByLineTag: Option<unsafe extern "C" fn(line: *mut c_void) -> *mut Mobj>,
	P_TeleportMove:      Option<unsafe extern "C" fn(thing: *mut Mobj, x: fixed, y: fixed) -> bool>,
	S_StartSound: 		 Option<unsafe extern "C" fn(origin_p: *const Mobj, sfx_id: SfxEnum)>,
    P_PlayerThink: 		 Option<unsafe extern "C" fn(player: *mut Player)>,
	P_MovePsprites:      Option<unsafe extern "C" fn(player: *mut Player)>,
	P_CalcHeight:        Option<unsafe extern "C" fn(player: *mut Player)>,
    P_UpdateSpecials:    Option<unsafe extern "C" fn()>,
    P_RespawnSpecials:   Option<unsafe extern "C" fn()>,
	P_SetMobjState:      Option<unsafe extern "C" fn(mobj: *mut c_void, state: StateEnum) -> bool>,
	P_GetMobjState:      Option<unsafe extern "C" fn(mobj: *mut c_void) -> StateEnum>,
	P_SpawnMobj:         Option<unsafe extern "C" fn(x: fixed, y: fixed, z: fixed, obj_type: MobjType) -> *mut Mobj>,

	// This should lie in the common library, but it's easier to put it here for now.
	Z_Free: Option<unsafe extern "C" fn(void_ptr: *mut c_void)>,
	I_Error: Option<unsafe extern "C" fn(str: *const std::ffi::c_char)>,
}

#[allow(nonstandard_style)]
impl Interface {
	pub fn P_TeleportByLineTag(&self, line: *mut c_void) -> *mut Mobj {
		unsafe {
			(self.P_TeleportByLineTag.as_ref().unwrap())(line)
		}
	}

	pub fn P_TeleportMove(&self, thing: *mut Mobj, x: fixed, y: fixed) -> bool {
		unsafe {
			(self.P_TeleportMove.as_ref().unwrap())(thing, x, y)
		}
	}

	pub fn S_StartSound(&self, origin_p: *const Mobj, sfx_id: SfxEnum) {
		unsafe {
			(self.S_StartSound.as_ref().unwrap())(origin_p, sfx_id)
		}
	}

	pub fn P_PlayerThink(&self, player: *mut Player) {
		unsafe {
			(self.P_PlayerThink.as_ref().unwrap())(player)
		}
	}

	pub fn P_MovePsprites(&self, player: *mut Player) {
		unsafe {
			(self.P_MovePsprites.as_ref().unwrap())(player)
		}
	}

	pub fn P_CalcHeight(&self, player: *mut Player) {
		unsafe {
			(self.P_CalcHeight.as_ref().unwrap())(player)
		}
	}

	pub fn P_UpdateSpecials(&self) {
		unsafe {
			(self.P_UpdateSpecials.as_ref().unwrap())()
		}
	}

	pub fn P_RespawnSpecials(&self) {
		unsafe {
			(self.P_RespawnSpecials.as_ref().unwrap())()
		}
	}

	pub fn P_SetMobjState(&self, mobj: *mut c_void, state: StateEnum) -> bool {
		unsafe {
			(self.P_SetMobjState.as_ref().unwrap())(mobj, state)
		}
	}

	pub fn P_GetMobjState(&self, mobj: *mut c_void) -> StateEnum {
		unsafe {
			(self.P_GetMobjState.as_ref().unwrap())(mobj)
		}
	}

	pub fn P_SpawnMobj(&self, x: fixed, y: fixed, z: fixed, obj_type: MobjType) -> *mut Mobj {
		unsafe {
			(self.P_SpawnMobj.as_ref().unwrap())(x, y, z, obj_type)
		}
	}

	pub fn Z_Free(&self, void_ptr: *mut c_void) {
		unsafe {
			(self.Z_Free.as_ref().unwrap())(void_ptr)
		}
	}

	pub fn I_Error(&self, str: &CString) {
		unsafe {
			(self.I_Error.as_ref().unwrap())(str.as_ptr())
		}
	}
}

#[unsafe(no_mangle)]
pub static mut INTERFACE: Interface = unsafe { std::mem::zeroed() };