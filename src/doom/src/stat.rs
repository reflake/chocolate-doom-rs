use crate::{defs::MAX_PLAYERS, player::Player};
use common::{bool::{FALSE, bool32}, mode::GameVersion};

#[unsafe(no_mangle)]
pub static mut paused: bool32 = FALSE;

#[unsafe(no_mangle)]
pub static mut netgame: bool32 = FALSE;

#[unsafe(no_mangle)]
pub static mut menuactive: bool32 = FALSE;

#[unsafe(no_mangle)]
pub static mut demoplayback: bool32 = FALSE;

#[unsafe(no_mangle)]
pub static mut consoleplayer: i32 = 0;

#[unsafe(no_mangle)]
pub static mut playeringame: [bool32; MAX_PLAYERS] = [FALSE; MAX_PLAYERS];

#[unsafe(no_mangle)]
pub static mut players: [Player; MAX_PLAYERS] = unsafe { std::mem::zeroed() };

#[unsafe(no_mangle)]
pub static mut leveltime: i32 = 0;

#[unsafe(no_mangle)]
pub static mut gameversion: GameVersion = GameVersion::exe_final2;