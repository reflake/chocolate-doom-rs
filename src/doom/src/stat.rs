use crate::{defs::MAX_PLAYERS, player::Player};

#[unsafe(no_mangle)]
pub static mut paused: bool = false;

#[unsafe(no_mangle)]
pub static mut netgame: bool = false;

#[unsafe(no_mangle)]
pub static mut menuactive: bool = false;

#[unsafe(no_mangle)]
pub static mut demoplayback: bool = false;

#[unsafe(no_mangle)]
pub static mut consoleplayer: i32 = 0;

#[unsafe(no_mangle)]
pub static mut playeringame: [u32; MAX_PLAYERS] = [0; MAX_PLAYERS];

#[repr(C)]
#[unsafe(no_mangle)]
pub static mut players: [Player<'_>; MAX_PLAYERS] = unsafe { std::mem::zeroed() };

#[unsafe(no_mangle)]
pub static mut leveltime: i32 = 0;