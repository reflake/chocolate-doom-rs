use bitflags::bitflags;

//
// DESCRIPTION:
//	System specific interface stuff.
//

// The data sampled per tick (single player)
// and transmitted to other peers (multiplayer).
// Mainly movements/button commands per game tick,
// plus a checksum for internal state consistency.
#[repr(C)]
pub struct TickCmd
{
    pub forward_move: i8,	// *2048 for move
    pub side_move: i8,		// *2048 for move
    pub angle_turn: i16,     // <<16 for angle delta
    pub chat_char: u8,
    pub buttons: ButtonCode,
    // villsa [STRIFE] according to the asm,
    // consistancy is a short, not a byte
    pub consistancy: u8,    // checks for net game

    // villsa - Strife specific:
    pub buttons2: u8,
    pub inventory: i32,
   
    // Heretic/Hexen specific:
    pub look_fly: u8,       // look/fly up/down/centering
	pub arti: u8           // artitype_t to use
}

//
// Button/action code definitions.
//
bitflags! {
	#[repr(C)]
	#[allow(nonstandard_style, dead_code)]
	pub struct ButtonCode: u8
	{
		// Press "Fire".
		const BT_ATTACK		= 1;
		// Use button, to open doors, activate switches.
		const BT_USE		= 2;

		// Flag: game events, not really buttons.
		const BT_SPECIAL		= 128;
		const BT_SPECIALMASK	= 3;
		
		// Flag, weapon change pending.
		// If true, the next 3 bits hold weapon num.
		const BT_CHANGE		= 4;
		// The 3bit weapon mask and shift, convenience.
		const BT_WEAPONMASK	= (8+16+32);
		const BT_WEAPONSHIFT	= 3;

		// Pause the game.
		const BTS_PAUSE		= 1;
		// Save the game at each console.
		const BTS_SAVEGAME	= 2;

		// Savegame slot numbers
		//  occupy the second byte of buttons.    
		const BTS_SAVEMASK	= (4+8+16);
		const BTS_SAVESHIFT = 2;
	
	}
}

impl TickCmd
{
	pub fn moving(&self) -> bool
	{
		self.forward_move != 0 || self.side_move != 0
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn tickcmd_size_equals_16() {
		assert_eq!(std::mem::size_of::<TickCmd>(), 16);
	}
}