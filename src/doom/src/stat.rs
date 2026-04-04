use crate::{defs::MAX_PLAYERS, player::Player};

unsafe extern "C" {
    pub static mut paused: bool;
    pub static mut netgame: bool;
    pub static mut menuactive: bool;
    pub static mut demoplayback: bool;
    pub static mut consoleplayer: i32;
    pub static mut playeringame: [u32; MAX_PLAYERS];
    pub static mut players: [Player; MAX_PLAYERS];
    pub static mut leveltime: i32;
}