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
    pub buttons: u8,
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

impl TickCmd
{
	pub fn moving(&self) -> bool
	{
		self.forward_move != 0 || self.side_move != 0
	}
}