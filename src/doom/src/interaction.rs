use std::{ffi::CString, ops::Not};

use common::{bool::{FALSE, TRUE, bool32}, limits, mode::SkillLevel};

use crate::{defs::{Card, PowerDuration, PowerType}, external::INTERFACE, mobj::Flags, player::{ArmorType, Player}, sounds::{SfxEnum, st_emit_sound}, stat::*, weapons::{AmmoType, GetWeaponInfo, NUM_OF_AMMO_TYPES, WeaponType}};

//
// DESCRIPTION:
//	Handling interactions (i.e., collisions).
//

const BONUSADD: i32 = 6;

// a weapon is found with two clip loads,
// a big item has five clip loads
const MAX_AMMO: [i32; NUM_OF_AMMO_TYPES] = [200, 50, 300, 50];
const CLIP_AMMO: [i32; NUM_OF_AMMO_TYPES] = [10, 4, 20, 1];

//
// GET STUFF
//

//
// P_GiveAmmo
// Num is the number of clip loads,
// not the individual count (0= 1/2 clip).
// Returns false if the ammo can't be picked up at all
//

#[allow(nonstandard_style, static_mut_refs)]
#[unsafe(no_mangle)]
pub extern "C" fn P_GiveAmmo( player: &mut Player, ammo: AmmoType, mut num: i32 ) -> bool32 {

    if ammo == AmmoType::am_noammo {
		return FALSE;
	}

    if ammo >= AmmoType::NUM_AMMO {

		let msg = format!("P_GiveAmmo: bad type {}", ammo as u32);

		unsafe { INTERFACE.I_Error(&CString::new(msg).unwrap()); }
	}
		
    if player.ammo[ammo as usize] == player.max_ammo[ammo as usize] {
		return FALSE;
	}
		
    if num > 0 {
		num *= CLIP_AMMO[ammo as usize];
    } else {
		num = CLIP_AMMO[ammo as usize] / 2;
    }
    
	unsafe {
		if gameskill == SkillLevel::sk_baby
			|| gameskill == SkillLevel::sk_nightmare
		{
			// give double ammo in trainer mode,
			// you'll need in nightmare
			num <<= 1;
		}
	}
    
    let oldammo = player.ammo[ammo as usize];
    player.ammo[ammo as usize] += num;

    if player.ammo[ammo as usize] > player.max_ammo[ammo as usize] {
        player.ammo[ammo as usize] = player.max_ammo[ammo as usize];
    }

    // If non zero ammo, 
    // don't change up weapons,
    // player was lower on purpose.
    if oldammo != 0 {
        return TRUE;
    }

    // We were down to zero,
    // so select a new weapon.
    // Preferences are not user selectable.
    match ammo {
      AmmoType::am_clip => {
		if player.ready_weapon == WeaponType::wp_fist {
			if player.owns_weapon(WeaponType::wp_chaingun) {
				player.pending_weapon = WeaponType::wp_chaingun;
			} else {
				player.pending_weapon = WeaponType::wp_pistol;
			}
		}
	},
	
      AmmoType::am_shell => {
		if player.ready_weapon == WeaponType::wp_fist
			|| player.ready_weapon == WeaponType::wp_pistol
		{
			if player.owns_weapon(WeaponType::wp_shotgun) {
				player.pending_weapon = WeaponType::wp_shotgun;
			}
		}
	},
	
      AmmoType::am_cell => {
		if player.ready_weapon == WeaponType::wp_fist
			|| player.ready_weapon == WeaponType::wp_pistol
		{
			if player.owns_weapon(WeaponType::wp_plasma) {
				player.pending_weapon = WeaponType::wp_plasma;
			}
		}
	},
	
      AmmoType::am_misl => {
		if player.ready_weapon == WeaponType::wp_fist
		{
		    if player.owns_weapon(WeaponType::wp_missile) {
				player.pending_weapon = WeaponType::wp_missile;
		    }
		}
      },
      _ => ()
    }
	
    TRUE
}


// P_GiveWeapon
// The weapon name may have a MF_DROPPED flag ored in.
//
#[allow(nonstandard_style)]
#[unsafe(no_mangle)]
pub extern "C" fn P_GiveWeapon(player: &mut Player, weapon: WeaponType, dropped: bool32 ) -> bool32
{
	unsafe {
		let mut gaveammo: bool = false;
		let mut gaveweapon: bool = false;

		let info = GetWeaponInfo(weapon).unwrap();
		
		if netgame.into() && deathmatch != 2 && dropped.not().into()
		{
			// leave placed weapons forever on net games
			if player.owns_weapon(weapon) {
				return FALSE;
			}

			player.bonus_count += BONUSADD;
			player.weapon_owned[weapon as usize] = TRUE;

			if deathmatch != 0 {
				P_GiveAmmo (player, info.ammo, 5);
			} else {
				P_GiveAmmo (player, info.ammo, 2);
			}

			player.pending_weapon = weapon;

			if std::ptr::addr_eq(player, &players[consoleplayer as usize]) {
				st_emit_sound(SfxEnum::sfx_wpnup);
			}
			return FALSE;
		}
			
		if info.ammo != AmmoType::am_noammo
		{
			// give one clip with a dropped weapon,
			// two clips with a found weapon
			if dropped.into() {
				gaveammo = P_GiveAmmo (player, info.ammo, 1).into();
			} else {
				gaveammo = P_GiveAmmo (player, info.ammo, 2).into();
			}
		} else {
			gaveammo = false;
		}
			
		if player.owns_weapon(weapon) {
			gaveweapon = false;
		}
		else
		{
			gaveweapon = true;
			player.weapon_owned[weapon as usize] = TRUE;
			player.pending_weapon = weapon;
		}
		
		return (gaveweapon || gaveammo).into();
	}
}

//
// P_GiveArmor
// Returns false if the armor is worse
// than the current armor.
//
#[allow(nonstandard_style)]
#[unsafe(no_mangle)]
pub extern "C" fn P_GiveArmor(player: &mut Player, armor_type: ArmorType) -> bool32 {
	let hits = match armor_type {
		ArmorType::Medium => 100,
		ArmorType::Heavy => 200,
		_ => 0
	};

	if hits <= player.armor_points {
		return FALSE;
	}

	player.armor_points = hits;
	player.armor_type = armor_type;
	TRUE
}

#[allow(nonstandard_style)]
#[unsafe(no_mangle)]
pub extern "C" fn P_GiveCard(player: &mut Player, card: Card)
{
    if player.cards[card as usize] == TRUE {
		return;
	}
    
    player.bonus_count = BONUSADD;
    player.cards[card as usize] = TRUE;
}

#[allow(nonstandard_style)]
#[unsafe(no_mangle)]
pub extern "C" fn P_GivePower(player: &mut Player, power: PowerType) -> bool32 {

	player.powers[power as usize] = match power {
		PowerType::Invulnerability => PowerDuration::INVULNTICS.into(),
		PowerType::Invisibility => {
			player.mobj().unwrap().flags |= Flags::SHADOW;
			PowerDuration::INVISTICS.into()
		},
		PowerType::IronFeet => PowerDuration::IRONTICS.into(),
		PowerType::Infrared => PowerDuration::INFRATICS.into(),
		PowerType::Strength => {
			P_GiveBody(player, 100);
			
			1
		},
		_ => {
			if player.powers[power as usize] != 0 {
				return FALSE;
			}

			1
		}
	};

    TRUE
}

//
// P_GiveBody
// Returns false if the body isn't needed at all
//
#[allow(nonstandard_style)]
#[unsafe(no_mangle)]
pub extern "C" fn P_GiveBody(player: &mut Player, num: i32) -> bool32 {
    if player.health >= limits::MAX_HEALTH {
        return FALSE;
    }

    player.health = player.health.wrapping_add(num);
    if player.health > limits::MAX_HEALTH {
        player.health = limits::MAX_HEALTH;
    }

    player.mobj().unwrap().health = player.health;

	TRUE
}