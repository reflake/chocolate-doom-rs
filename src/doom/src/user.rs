use common::fixed::fixed;
use common::tickcmd::TickCmd;
use common::trigonometry::{ang};

use crate::info::StateEnum;
use crate::mobj::{P_MobjStateEqual, P_SetMobjState};

//
// P_Thrust
// Moves the given origin along a given angle.
//
fn thrust(move_vel: &mut (fixed, fixed), angle: ang, mov: fixed)
{
	move_vel.0 += mov * angle.fine_cosine();
	move_vel.1 += mov * angle.fine_sine();
}

#[unsafe(no_mangle)]
pub extern "C" fn PlayerOnGround(player_z: fixed, player_floor_z: fixed) -> bool
{
	player_z <= player_floor_z
}

#[unsafe(no_mangle)]
pub extern "C" fn MovePlayer(
	player_mobj: *mut std::ffi::c_void, 
	move_vel: &mut (fixed, fixed), 
	angle: &mut ang, 
	cmd: &TickCmd, 
	on_ground: bool)
{
	*angle += ang::from_hi(cmd.angle_turn);

	if on_ground {
		thrust(move_vel, *angle, fixed(cmd.forward_move as i32 * 2048));
		thrust(move_vel, *angle - ang::from_hi(0x4000), fixed(cmd.side_move as i32 * 2048));
	}

	unsafe {
		if cmd.moving() 
			&& P_MobjStateEqual(player_mobj, StateEnum::S_PLAY)
		{
			P_SetMobjState(player_mobj, StateEnum::S_PLAY_RUN1);
		}
	}
}