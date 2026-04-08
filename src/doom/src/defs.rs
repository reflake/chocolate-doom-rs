use common::timer::TICKRATE;

pub const MAX_PLAYERS: usize = 4;

//
// Key cards.
//
#[allow(dead_code)]
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Card
{
    BlueCard,
    YellowCard,
    RedCard,
    BlueSkull,
    YellowSkull,
    RedSkull,
    
    NUMCARDS
}


pub const NUM_OF_CARD_TYPES: usize = Card::NUMCARDS as usize;

// Power up artifacts.
#[allow(dead_code)]
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PowerType
{
    Invulnerability,
    Strength,
    Invisibility,
    IronFeet,
    AllMap,
    Infrared,

    NUMPOWERS
}

pub const NUM_OF_POWER_TYPES: usize = PowerType::NUMPOWERS as usize;

// Power up durations,
//  how many seconds till expiration,
//  assuming TICRATE is 35 ticks/second.
//
pub struct PowerDuration(u32);

impl From<PowerDuration> for u32 {
	fn from(value: PowerDuration) -> Self {
		value.0
	}
}

impl PowerDuration {
    pub const INVULNTICS: PowerDuration	= PowerDuration(30  * TICKRATE);
    pub const INVISTICS: PowerDuration	= PowerDuration(60  * TICKRATE);
    pub const INFRATICS: PowerDuration	= PowerDuration(120 * TICKRATE);
    pub const IRONTICS: PowerDuration	= PowerDuration(60  * TICKRATE);
}