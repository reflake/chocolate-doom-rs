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
    pub forwardmove: i8,	// *2048 for move
    pub sidemove: i8,		// *2048 for move
    pub angleturn: i16,     // <<16 for angle delta
    pub chatchar: u8,
    pub buttons: u8,
    // villsa [STRIFE] according to the asm,
    // consistancy is a short, not a byte
    pub consistancy: u8,    // checks for net game

    // villsa - Strife specific:
    pub buttons2: u8,
    pub inventory: i32,
   
    // Heretic/Hexen specific:
    pub lookfly: u8,       // look/fly up/down/centering
	pub arti: u8           // artitype_t to use
}