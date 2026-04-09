// Doom External hehe

use common::{bool::bool32, fixed::fixed, sounds::SfxInfo};
use std::ffi::{CString, c_void};

use crate::{info::StateEnum, mobj::{Mobj, MobjType}, player::Player, sounds::SfxEnum};

// An interface to the external C library functions.
#[repr(C)]
#[allow(nonstandard_style)]
pub struct Interface {
	
	P_TeleportByLineTag: Option<unsafe extern "C" fn(line: *mut c_void) -> *mut Mobj>,
	P_TeleportMove:      Option<unsafe extern "C" fn(thing: *mut Mobj, x: fixed, y: fixed) -> bool32>,
    P_PlayerThink: 		 Option<unsafe extern "C" fn(player: *mut Player)>,
	P_MovePsprites:      Option<unsafe extern "C" fn(player: *mut Player)>,
    P_UpdateSpecials:    Option<unsafe extern "C" fn()>,
    P_RespawnSpecials:   Option<unsafe extern "C" fn()>,
	P_SetMobjState:      Option<unsafe extern "C" fn(mobj: *mut c_void, state: StateEnum) -> bool32>,
	P_GetMobjState:      Option<unsafe extern "C" fn(mobj: *mut c_void) -> StateEnum>,
	P_SpawnMobj:         Option<unsafe extern "C" fn(x: fixed, y: fixed, z: fixed, obj_type: MobjType) -> *mut Mobj>,

	// This should lie in the common library, but it's easier to put it here for now.
	Z_Free: Option<unsafe extern "C" fn(void_ptr: *mut c_void)>,
	I_Error: Option<unsafe extern "C" fn(str: *const std::ffi::c_char)>,
	I_StartSound: Option<unsafe extern "C" fn(sfxinfo: *const c_void, channel: i32, vol: i32, sep: i32, pitch: i32) -> i32>,
	I_GetSfxLumpNum: Option<unsafe extern "C" fn(sfxinfo: *const c_void) -> i32>,
	I_StopSound: Option<unsafe extern "C" fn(handle: i32)>,
	I_SoundIsPlaying: Option<unsafe extern "C" fn(handle: i32) -> bool32>,
	I_UpdateSounds: Option<unsafe extern "C" fn()>,
	I_UpdateSoundParams: Option<unsafe extern "C" fn(handle: i32, vol: i32, sep: i32)>,
}

#[allow(nonstandard_style)]
impl Interface {
	pub fn P_TeleportByLineTag(&self, line: *mut c_void) -> *mut Mobj {
		unsafe {
			(self.P_TeleportByLineTag.as_ref().unwrap())(line)
		}
	}

	pub fn P_TeleportMove(&self, thing: *mut Mobj, x: fixed, y: fixed) -> bool32 {
		unsafe {
			(self.P_TeleportMove.as_ref().unwrap())(thing, x, y)
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

	pub fn P_SetMobjState(&self, mobj: *mut c_void, state: StateEnum) -> bool32 {
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

	pub fn I_StartSound(&self, sfxinfo: &SfxInfo, channel: i32, vol: i32, sep: i32, pitch: i32) -> i32 {
		unsafe {
			(self.I_StartSound.as_ref().unwrap())(std::mem::transmute(sfxinfo), channel, vol, sep, pitch)
		}
	}

	pub fn I_GetSfxLumpNum(&self, sfxinfo: &SfxInfo) -> i32 {
		unsafe {
			(self.I_GetSfxLumpNum.as_ref().unwrap())(std::mem::transmute(sfxinfo))
		}
	}

	pub fn I_StopSound(&self, handle: i32) {
		unsafe {
			(self.I_StopSound.as_ref().unwrap())(handle)
		}
	}

	pub fn I_SoundIsPlaying(&self, handle: i32) -> bool32 {
		unsafe {
			(self.I_SoundIsPlaying.as_ref().unwrap())(handle)
		}
	}

	pub fn I_UpdateSounds(&self) {
		unsafe {
			(self.I_UpdateSounds.as_ref().unwrap())()
		}
	}

	pub fn I_UpdateSoundParams(&self, handle: i32, vol: i32, sep: i32) {
		unsafe {
			(self.I_UpdateSoundParams.as_ref().unwrap())(handle, vol, sep)
		}
	}
}

#[unsafe(no_mangle)]
pub static mut INTERFACE: Interface = unsafe { std::mem::zeroed() };

#[macro_export]
macro_rules! I_Error {
	($($arg:tt)*) => {
		unsafe {
			use std::ffi::CString;
			INTERFACE.I_Error(&CString::new(format!($($arg)*)).unwrap());
		}
	};
}