use std::ffi::CStr;

use common::{fixed::fixed, tickcmd::TickCmd};

use crate::mobj::Mobj;

//
// Extended player object info: player_t
//
#[repr(C)]
pub struct Player
{
    mo: *mut Mobj,
    //playerstate_t	
    playerstate: i32,
    cmd: TickCmd,

    // Determine POV,
    //  including viewpoint bobbing during movement.
    // Focal origin above r.z
    pub viewz: fixed,
    // Base height above floor for viewz.
    pub viewheight: fixed,
    // Bob/squat speed.
    deltaviewheight: fixed,
    // bounded/scaled total momentum.
    bob: fixed,

    // This is only used between levels,
    // mo->health is used during levels.
    health: i32,
    armorpoints: i32,
    // Armor type is 0-2.
    armortype: i32,

    // Power ups. invinc and invis are tic counters.
    powers: [i32; 6],
    cards: [i32; 6],
    backpack: bool,
    
    // Frags, kills of other players.
    frags: [i32; 4],

    //weapontype_t	
    readyweapon: i32,
    
    // Is wp_nochange if not changing.
    //weapontype_t	
    pendingweapon: i32,

    weaponowned: [i32; 9],
    ammo: [i32; 4],
    maxammo: [i32; 4],

    // True if button down last tic.
    attackdown: i32,
    usedown: i32,

    // Bit flags, for cheats and debug.
    // See cheat_t, above.
    cheats: i32,

    // Refired shots are less accurate.
    refire: i32,

     // For intermission stats.
    killcount: i32,
    itemcount: i32,
    secretcount: i32,

    // Hint messages.
    message: *const std::ffi::c_char,
    
    // For screen flashing (red or bright).
    damagecount: i32,
    bonuscount: i32,

    // Who did damage (NULL for floors/ceilings).
    attacker: *mut Mobj,
    
    // So gun flashes light up areas.
    extralight: i32,

    // Current PLAYPAL, ???
    //  can be set to REDCOLORMAP for pain, etc.
    fixedcolormap: i32,

    // Player skin colorshift,
    //  0-3 for which color to draw player.
    colormap: i32,

    // Overlay view sprites (gun, etc).
    //pspdef_t[2]
    psprites: [u8; 48],

    // True if secret level has been done.
    didsecret: bool,
}