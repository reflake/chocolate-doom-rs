use bitflags::bitflags;
use common::{fixed::fixed, ptr_as_ref_mut, trigonometry::ang, vector::{concrete::{vec2, vec3}, vec}};

use crate::{external::INTERFACE, info::StateEnum, player::Player, sounds::SfxEnum, tics::Thinker};

#[unsafe(no_mangle)]
pub static mut onground: bool = false;

#[allow(static_mut_refs, dead_code)]
impl Mobj {
	pub fn spawn(position: vec3, obj_type: MobjType) -> &'static Mobj {
		unsafe {
			let p = position;
			&mut *INTERFACE.P_SpawnMobj(p.x, p.y, p.z, obj_type)
		}
	}
	
	pub fn spawn_mut(position: vec3, obj_type: MobjType) -> &'static mut Mobj {
		unsafe {
			let p = position;
			&mut *INTERFACE.P_SpawnMobj(p.x, p.y, p.z, obj_type)
		}
	}

	pub fn player<'a>(&mut self) -> Option<&'a mut Player> {
		ptr_as_ref_mut(self.player)
	}

	// Moves the given origin along a given angle.
	pub fn thrust(&mut self, angle: ang, mov: fixed) {
		self.momentum.x += mov * angle.fine_cosine();
		self.momentum.y += mov * angle.fine_sine();
	}

	pub fn set_state(&mut self, state: StateEnum) {
		unsafe {
			INTERFACE.P_SetMobjState(std::mem::transmute(self), state);
		}
	}

	pub fn get_state(&self) -> StateEnum {
		unsafe {
			INTERFACE.P_GetMobjState(std::mem::transmute(self))
		}
	}

    #[allow(static_mut_refs)]
    pub fn spawn_missile(&self, target: &mut Mobj, missile_type: MobjType) -> Option<&'static mut Mobj> {
        unsafe {
			INTERFACE.P_SpawnMissile (self, target, missile_type).as_mut()
        }
    }
}

#[repr(C, packed)]
pub struct MapThing
{
	position: vec<i16, 2>,
    angle: i16,
    obj_type: i16,
    options: i16,
}

#[repr(C)]
pub struct Info
{
	pub doomednum: i32,
	pub spawnstate: i32,
	pub spawnhealth: i32,
	pub seestate: i32,
	pub seesound: i32,
	pub reactiontime: i32,
	pub attacksound: SfxEnum,
	pub painstate: i32,
	pub painchance: i32,
	pub painsound: i32,
	pub meleestate: i32,
	pub missilestate: i32,
	pub deathstate: i32,
	pub xdeathstate: i32,
	pub deathsound: i32,
	pub speed: fixed,
	pub radius: i32,
	pub height: i32,
	pub mass: i32,
	pub damage: i32,
	pub activesound: i32,
	pub flags: i32,
	pub raisestate: i32,
}

#[repr(C)]
pub struct Mobj
{
    // List: thinker links.
    pub thinker: Thinker,

    // Info for drawing: position.
    pub position: vec3,

    // More list: links in sector (if needed)
    snext: *mut Mobj,
    sprev: *mut Mobj,

    //More drawing info: to determine current sprite.
    pub angle: ang,	            // orientation
    // spriteenum_t
    pub sprite: std::ffi::c_int,// used to find patch_t and flip value
    pub frame: i32,	            // might be ORed with FF_FULLBRIGHT

    // Interaction info, by BLOCKMAP.
    // Links in blocks (if needed).
    bnext: *mut Mobj,
    bprev: *mut Mobj,
    
    //struct subsector_s*	
    subsector: *mut std::ffi::c_void,

    // The closest interval over all contacted Sectors.
	pub floorz:   fixed,
	pub ceilingz: fixed,

    // For movement checking.
	pub radius: fixed,
	pub height: fixed,	

    // Momentums, used to update position.
	pub momentum: vec3,

    // If == validcount, already checked.
    pub validcount: i32,

	pub obj_type: MobjType,
    pub info: *mut Info,	// &mobjinfo[mobj->type]
    
    tics: i32,	// state tic counter
    //state_t*		
    pub state: *mut std::ffi::c_void,
    pub flags: Flags,
    pub health: i32,

    // Movement direction, movement generation (zig-zagging).
    pub movedir:   i32,	// 0-7
    pub movecount: i32,	// when 0, select a new dir

    // Thing being chased/attacked (or NULL),
    // also the originator for missiles.	
    pub target: *mut Mobj,

    // Reaction time: if non 0, don't attack yet.
    // Used by player to freeze a bit after teleporting.
    pub reactiontime: i32,  

    // If >0, the target will be chased
    // no matter what (even if shot)
    pub threshold: i32,

    // Additional info record for player avatars only.
    // Only valid if type == MT_PLAYER
    player: *mut Player,

    // Player number last looked for.
    pub lastlook: i32,

    // For nightmare respawn.
    // mapthing_t
	pub spawnpoint: MapThing,

    // Thing being chased/attacked for tracers.
    pub tracer: *mut Mobj,
    
}

impl Mobj {
	pub fn forward_xy(&self) -> vec2 {
		vec2{ 
			x: self.angle.fine_cosine(),
			y: self.angle.fine_sine(),
		}
	}

	// Certain functions assume that a mobj_t pointer is non-NULL,
	// causing a crash in some situations where it is NULL.  Vanilla
	// Doom did not crash because of the lack of proper memory 
	// protection. This function substitutes NULL pointers for
	// pointers to a dummy mobj, to avoid a crash.
	#[allow(static_mut_refs)]
	pub fn P_SubstNullMobj<'a>(mobj: *mut Mobj) -> &'a mut Mobj
	{
		unsafe {
			let Some(mobj) = mobj.as_mut()
			else
			{
				static mut DUMMY_MOBJ: Mobj = unsafe { std::mem::zeroed() };

				DUMMY_MOBJ.position = vec3::ZERO;
				DUMMY_MOBJ.flags = Flags::empty();

				return &mut DUMMY_MOBJ
			};

			mobj
		}
	}
}

//
// Misc. mobj flags
//
bitflags! {
    #[repr(C)]
	#[allow(nonstandard_style, dead_code)]
	pub struct Flags : u32
	{
		// Call P_SpecialThing when touched.
		const SPECIAL	 = 1;
		// Blocks.
		const SOLID		 = 2;
		// Can be hit.
		const SHOOTABLE	 = 4;
		// Don't use the sector links (invisible but touchable).
		const NOSECTOR	 = 8;
		// Don't use the blocklinks (inert but displayable)
		const NOBLOCKMAP = 16;                    

		// Not to be activated by sound, deaf monster.
		const AMBUSH		= 32;
		// Will try to attack right back.
		const JUSTHIT		= 64;
		// Will take at least one step before attacking.
		const JUSTATTACKED	= 128;
		// On level spawning (initial position),
		//  hang from ceiling instead of stand on floor.
		const SPAWNCEILING	= 256;
		// Don't apply gravity (every tic),
		//  that is, object will float, keeping current height
		//  or changing it actively.
		const NOGRAVITY		= 512;

		// Movement flags.
		// This allows jumps from high places.
		const DROPOFF		= 0x400;
		// For players, will pick up items.
		const PICKUP		= 0x800;
		// Player cheat. ???
		const NOCLIP		= 0x1000;
		// Player: keep info about sliding along walls.
		const SLIDE			= 0x2000;
		// Allow moves to any height, no gravity.
		// For active floaters, e.g. cacodemons, pain elementals.
		const FLOAT			= 0x4000;
		// Don't cross lines
		//   ??? or look at heights on teleport.
		const TELEPORT		= 0x8000;
		// Don't hit same species, explode on block.
		// Player missiles as well as fireballs of various kinds.
		const MISSILE		= 0x10000;	
		// Dropped by a demon, not level spawned.
		// E.g. ammo clips dropped by dying former humans.
		const DROPPED		= 0x20000;
		// Use fuzzy draw (shadow demons or spectres),
		//  temporary player invisibility powerup.
		const SHADOW		= 0x40000;
		// Flag: don't bleed when shot (use puff),
		//  barrels and shootable furniture shall not bleed.
		const NOBLOOD		= 0x80000;
		// Don't stop moving halfway off a step,
		//  that is, have dead bodies slide down all the way.
		const CORPSE		= 0x100000;
		// Floating to a height for a move, ???
		//  don't auto float to target's height.
		const INFLOAT		= 0x200000;

		// On kill, count this enemy object
		//  towards intermission kill total.
		// Happy gathering.
		const COUNTKILL	= 0x400000;
		
		// On picking up, count this item object
		//  towards intermission item total.
		const COUNTITEM	= 0x800000;

		// Special handling: skull in flight.
		// Neither a cacodemon nor a missile.
		const SKULLFLY		= 0x1000000;

		// Don't spawn this object
		//  in death match mode (e.g. key cards).
		const NOTDMATCH    	= 0x2000000;

		// Player sprites in multiplayer modes are modified
		//  using an internal color lookup table for re-indexing.
		// If 0x4 0x8 or 0xc,
		//  use a translation table for player colormaps
		const TRANSLATION  	= 0xc000000;
		// Hmm ???.
		const TRANSSHIFT	= 26;
	}
}

#[allow(nonstandard_style, dead_code)]
#[repr(u32)]
pub enum MobjType {
    MT_PLAYER,
    MT_POSSESSED,
    MT_SHOTGUY,
    MT_VILE,
    MT_FIRE,
    MT_UNDEAD,
    MT_TRACER,
    MT_SMOKE,
    MT_FATSO,
    MT_FATSHOT,
    MT_CHAINGUY,
    MT_TROOP,
    MT_SERGEANT,
    MT_SHADOWS,
    MT_HEAD,
    MT_BRUISER,
    MT_BRUISERSHOT,
    MT_KNIGHT,
    MT_SKULL,
    MT_SPIDER,
    MT_BABY,
    MT_CYBORG,
    MT_PAIN,
    MT_WOLFSS,
    MT_KEEN,
    MT_BOSSBRAIN,
    MT_BOSSSPIT,
    MT_BOSSTARGET,
    MT_SPAWNSHOT,
    MT_SPAWNFIRE,
    MT_BARREL,
    MT_TROOPSHOT,
    MT_HEADSHOT,
    MT_ROCKET,
    MT_PLASMA,
    MT_BFG,
    MT_ARACHPLAZ,
    MT_PUFF,
    MT_BLOOD,
    MT_TFOG,
    MT_IFOG,
    MT_TELEPORTMAN,
    MT_EXTRABFG,
    MT_MISC0,
    MT_MISC1,
    MT_MISC2,
    MT_MISC3,
    MT_MISC4,
    MT_MISC5,
    MT_MISC6,
    MT_MISC7,
    MT_MISC8,
    MT_MISC9,
    MT_MISC10,
    MT_MISC11,
    MT_MISC12,
    MT_INV,
    MT_MISC13,
    MT_INS,
    MT_MISC14,
    MT_MISC15,
    MT_MISC16,
    MT_MEGA,
    MT_CLIP,
    MT_MISC17,
    MT_MISC18,
    MT_MISC19,
    MT_MISC20,
    MT_MISC21,
    MT_MISC22,
    MT_MISC23,
    MT_MISC24,
    MT_MISC25,
    MT_CHAINGUN,
    MT_MISC26,
    MT_MISC27,
    MT_MISC28,
    MT_SHOTGUN,
    MT_SUPERSHOTGUN,
    MT_MISC29,
    MT_MISC30,
    MT_MISC31,
    MT_MISC32,
    MT_MISC33,
    MT_MISC34,
    MT_MISC35,
    MT_MISC36,
    MT_MISC37,
    MT_MISC38,
    MT_MISC39,
    MT_MISC40,
    MT_MISC41,
    MT_MISC42,
    MT_MISC43,
    MT_MISC44,
    MT_MISC45,
    MT_MISC46,
    MT_MISC47,
    MT_MISC48,
    MT_MISC49,
    MT_MISC50,
    MT_MISC51,
    MT_MISC52,
    MT_MISC53,
    MT_MISC54,
    MT_MISC55,
    MT_MISC56,
    MT_MISC57,
    MT_MISC58,
    MT_MISC59,
    MT_MISC60,
    MT_MISC61,
    MT_MISC62,
    MT_MISC63,
    MT_MISC64,
    MT_MISC65,
    MT_MISC66,
    MT_MISC67,
    MT_MISC68,
    MT_MISC69,
    MT_MISC70,
    MT_MISC71,
    MT_MISC72,
    MT_MISC73,
    MT_MISC74,
    MT_MISC75,
    MT_MISC76,
    MT_MISC77,
    MT_MISC78,
    MT_MISC79,
    MT_MISC80,
    MT_MISC81,
    MT_MISC82,
    MT_MISC83,
    MT_MISC84,
    MT_MISC85,
    MT_MISC86,
    NUMMOBJTYPES

}