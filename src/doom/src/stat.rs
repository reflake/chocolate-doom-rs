use crate::{defs::MAX_PLAYERS, player::Player};
use common::{bool::{FALSE, bool32}, mode::{GameVersion, SkillLevel}};

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

#[unsafe(no_mangle)]
pub static mut gameskill: SkillLevel = SkillLevel::sk_baby;

#[unsafe(no_mangle)]
pub static mut deathmatch: i32 = 0; // only if started as net death 