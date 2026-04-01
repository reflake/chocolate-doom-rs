use common::fixed::fixed;
use common::tickcmd::TickCmd;
use common::trigonometry::{ang};

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
pub extern "C" fn MovePlayer(move_vel: &mut (fixed, fixed), angle: &mut ang, cmd: &TickCmd, on_ground: bool)
{
	*angle += ang::from_hi(cmd.angleturn);

	if on_ground {
		thrust(move_vel, *angle, fixed(cmd.forwardmove as i32 * 2048));
		thrust(move_vel, *angle - ang::from_hi(0x4000), fixed(cmd.sidemove as i32 * 2048));
	}
}