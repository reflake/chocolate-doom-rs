
use std::ptr;

use crate::info::StateEnum;

// The defined weapons,
//  including a marker indicating
//  user has not changed weapon.
#[allow(nonstandard_style)]
#[repr(u32)]
pub enum WeaponType
{
    wp_fist,
    wp_pistol,
    wp_shotgun,
    wp_chaingun,
    wp_missile,
    wp_plasma,
    wp_bfg,
    wp_chainsaw,
    wp_supershotgun,

    NUMWEAPONS,
    
    // No pending weapon change.
    wp_nochange

}


// Ammunition types defined.
#[allow(nonstandard_style)]
#[repr(u32)]
pub enum AmmoType
{
    am_clip,	// Pistol / chaingun ammo.
    am_shell,	// Shotgun / double barreled shotgun.
    am_cell,	// Plasma rifle, BFG.
    am_misl,	// Missile launcher.
    NUMAMMO,
    am_noammo	// Unlimited for chainsaw / fist.	
}

// Weapon info: sprite frames, ammunition use.
#[allow(nonstandard_style)]
#[repr(C)]
pub struct WeaponInfo
{
    ammo: AmmoType,
    upstate: StateEnum,
    downstate: StateEnum,
    readystate: StateEnum,
    atkstate: StateEnum,
    flashstate: StateEnum,
}

use AmmoType::*;
use WeaponType::*;
use StateEnum::*;

//
// PSPRITE ACTIONS for waepons.
// This struct controls the weapon animations.
//
// Each entry is:
//   ammo/amunition type
//  upstate
//  downstate
// readystate
// atkstate, i.e. attack/fire/hit frame
// flashstate, muzzle flash
//
static WEAPON_INFOS: [WeaponInfo; NUMWEAPONS as usize] = [
    WeaponInfo {
		// fist
		ammo: am_noammo,
		upstate: S_PUNCHUP,
		downstate: S_PUNCHDOWN,
		readystate: S_PUNCH,
		atkstate: S_PUNCH1,
		flashstate: S_NULL
    },	
    WeaponInfo {
		// pistol
		ammo: am_clip,
		upstate: S_PISTOLUP,
		downstate: S_PISTOLDOWN,
		readystate: S_PISTOL,
		atkstate: S_PISTOL1,
		flashstate: S_PISTOLFLASH
    },	
    WeaponInfo {
		// shotgun
		ammo: am_shell,
		upstate: S_SGUNUP,
		downstate: S_SGUNDOWN,
		readystate: S_SGUN,
		atkstate: S_SGUN1,
		flashstate: S_SGUNFLASH1
    },
    WeaponInfo {
		// chaingun
		ammo: am_clip,
		upstate: S_CHAINUP,
		downstate: S_CHAINDOWN,
		readystate: S_CHAIN,
		atkstate: S_CHAIN1,
		flashstate: S_CHAINFLASH1
    },
    WeaponInfo {
		// missile launcher
		ammo: am_misl,
		upstate: S_MISSILEUP,
		downstate: S_MISSILEDOWN,
		readystate: S_MISSILE,
		atkstate: S_MISSILE1,
		flashstate: S_MISSILEFLASH1
    },
    WeaponInfo {
		// plasma rifle
		ammo: am_cell,
		upstate: S_PLASMAUP,
		downstate: S_PLASMADOWN,
		readystate: S_PLASMA,
		atkstate: S_PLASMA1,
		flashstate: S_PLASMAFLASH1
    },
    WeaponInfo {
		// bfg 9000
		ammo: am_cell,
		upstate: S_BFGUP,
		downstate: S_BFGDOWN,
		readystate: S_BFG,
		atkstate: S_BFG1,
		flashstate: S_BFGFLASH1
    },
    WeaponInfo {
		// chainsaw
		ammo: am_noammo,
		upstate: S_SAWUP,
		downstate: S_SAWDOWN,
		readystate: S_SAW,
		atkstate: S_SAW1,
		flashstate: S_NULL
    },
    WeaponInfo {
		// super shotgun
		ammo: am_shell,
		upstate: S_DSGUNUP,
		downstate: S_DSGUNDOWN,
		readystate: S_DSGUN,
		atkstate: S_DSGUN1,
		flashstate: S_DSGUNFLASH1
    },	
];

#[unsafe(no_mangle)]
pub extern "C" fn GetWeaponInfo(weapon: WeaponType) -> *mut WeaponInfo
{
	ptr::addr_of!(WEAPON_INFOS[weapon as usize]) as *mut WeaponInfo
}