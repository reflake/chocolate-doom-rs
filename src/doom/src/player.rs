use bitflags::bitflags;
use common::{bool::{TRUE, bool32}, fixed::fixed, ptr_as_ref_mut, tickcmd::{ButtonCode, TickCmd}, trigonometry::{R_PointToAngle2, ang}};

use crate::{defs::{NUM_OF_CARD_TYPES, NUM_OF_POWER_TYPES}, external::INTERFACE, mobj::{Mobj, onground}, weapons::{NUM_OF_AMMO_TYPES, NUM_OF_WEAPON_TYPES, WeaponType}};

#[repr(C)]
pub struct Stub {
    arr: [u8; 24],
}

//
// Extended player object info: player_t
//
#[repr(C)]
pub struct Player
{
    mo: *mut Mobj,

    pub state: State,
    pub cmd: TickCmd,

    // Determine POV,
    //  including viewpoint bobbing during movement.
    // Focal origin above r.z
    pub viewz: fixed,
    // Base height above floor for viewz.
    pub viewheight: fixed,
    // Bob/squat speed.
    pub deltaviewheight: fixed,
    // bounded/scaled total momentum.
    pub bob: fixed,

    // This is only used between levels,
    // mo->health is used during levels.
    pub health: i32,
    pub armor_points: i32,
    // Armor type is 0-2.
    pub armor_type: ArmorType,

    // Power ups. invinc and invis are tic counters.
    pub powers: [u32; NUM_OF_POWER_TYPES],
    pub cards: [bool32; NUM_OF_CARD_TYPES],
    pub backpack: std::ffi::c_int,
    
    // Frags, kills of other players.
    pub frags: [i32; 4],

    pub ready_weapon: WeaponType,
    
    // Is wp_nochange if not changing.
    pub pending_weapon: WeaponType,

    pub weapon_owned: [bool32; NUM_OF_WEAPON_TYPES],
    pub ammo:   	 [i32; NUM_OF_AMMO_TYPES],
    pub max_ammo:	 [i32; NUM_OF_AMMO_TYPES],

    // True if button down last tic.
    pub attackdown: i32,
    pub usedown:    i32,

    // Bit flags, for cheats and debug.
    // See cheat_t, above.
    pub cheats: CheatFlags,

    // Refired shots are less accurate.
    pub refire: i32,

     // For intermission stats.
    pub killcount:   i32,
    pub itemcount:   i32,
    pub secretcount: i32,

    // Hint messages.
    pub message: *const std::ffi::c_char,
    
    // For screen flashing (red or bright).
    pub damagecount: i32,
    pub bonus_count:  i32,

    // Who did damage (NULL for floors/ceilings).
    attacker: *mut Mobj,
    
    // So gun flashes light up areas.
    pub extralight: i32,

    // Current PLAYPAL, ???
    //  can be set to REDCOLORMAP for pain, etc.
    pub fixedcolormap: i32,

    // Player skin colorshift,
    //  0-3 for which color to draw player.
    pub colormap: i64,

    // Overlay view sprites (gun, etc).
    //pspdef_t[2]
    pub psprites: [Stub; 2],

    // True if secret level has been done.
    pub didsecret: i32,
}

//
// Player states.
//
#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
pub enum State
{
    // Playing or camping.
    PST_LIVE,
    // Dead on the ground, view follows killer.
    PST_DEAD,
    // Ready to restart/respawn???
    PST_REBORN
}

#[repr(C)]
#[allow(non_camel_case_types, dead_code)]
pub enum ArmorType
{
	None,
	Medium,
	Heavy
}

// Player internal flags, for cheats and debug.
//
bitflags! {
	#[repr(C)]
	#[allow(nonstandard_style, dead_code)]
	pub struct CheatFlags : u32
	{
		// No clipping, walk through barriers.
		const CF_NOCLIP		= 1;
		// No damage, no health loss.
		const CF_GODMODE	= 2;
		// Not really a cheat, just a debug aid.
		const CF_NOMOMENTUM	= 4;
	}
}

impl Player
{
    pub const fn attacker(&self) -> Option<&mut Mobj> {
        ptr_as_ref_mut(self.attacker)
    }

    pub const fn mobj<'a>(&self) -> Option<&'a mut Mobj> {
        ptr_as_ref_mut(self.mo)
    }

	#[allow(static_mut_refs)]
    pub fn think(&mut self)
    {
        unsafe {
            INTERFACE.P_PlayerThink(std::mem::transmute(self));
        }
    }

	#[allow(static_mut_refs)]
	pub fn death_think(&mut self)
	{
		unsafe { INTERFACE.P_MovePsprites(&mut *self); }
		
		// fall to the ground
		if self.viewheight > fixed::from(6) {
			self.viewheight -= fixed::from(1);
		}

		if self.viewheight < fixed::from(6) {
			self.viewheight = fixed::from(6);
		}

		self.deltaviewheight = fixed::from(0);
		
        let self_mobj = self.mobj().unwrap();

		unsafe { 
			// TODO: remove this static variable in the future
			onground = self_mobj.position.z <= self_mobj.floorz;
			INTERFACE.P_CalcHeight(&mut *self); 
		}

		if let Some(attacker) = self.attacker() 
            && std::ptr::eq(self.attacker, self_mobj)
		{
			let angle = R_PointToAngle2(self_mobj.position.xy(), attacker.position.xy());
			let delta = angle - self_mobj.angle;
			
			if delta < ang::degree(5.0) || delta > ang::degree(-5.0)
			{
				// Looking at killer,
				//  so fade damage flash down.
				self_mobj.angle = angle;

				if self.damagecount != 0 {
					self.damagecount -= 1;
				}
			}
			else if delta < ang::degree(180.0) {
				self_mobj.angle += ang::degree(5.0);
			}
			else 
			{
				self_mobj.angle -= ang::degree(5.0);
			}
		}
		else if self.damagecount != 0 {
			self.damagecount -= 1;
		}
		
		if self.cmd.buttons.contains(ButtonCode::BT_USE) {
			self.state = State::PST_REBORN;
		}
	}

	pub fn owns_weapon(&self, weapon: WeaponType) -> bool {
		self.weapon_owned[weapon as usize] == TRUE
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn P_DeathThink(player: &mut Player)
{
	player.death_think();
}


#[cfg(test)]
mod test {
	use super::*;
	use rstest::rstest;
 
	#[rstest]
	fn player_size_equals_328() {
		assert_eq!(std::mem::size_of::<Player>(), 328);
	}
}