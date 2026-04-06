use common::fixed::fixed;
use common::trigonometry::{ang};

use crate::info::StateEnum;
use crate::player::Player;

//
// DESCRIPTION:
//	Player related stuff.
//	Bobbing POV/weapon, movement.
//	Pending weapon.
//

impl <'a> Player<'a> {
	pub fn move_player(&mut self, on_ground: bool) {

		self.mo.angle += ang::from_hi(self.cmd.angle_turn);

		if on_ground {
			self.mo.thrust(self.mo.angle, fixed(self.cmd.forward_move as i32 * 2048));
			self.mo.thrust(self.mo.angle - ang::degree(90.0), fixed(self.cmd.side_move as i32 * 2048));
		}

		if self.cmd.moving() 
			&& self.mo.get_state() == StateEnum::S_PLAY
		{
			self.mo.set_state(StateEnum::S_PLAY_RUN1);
		}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn PlayerOnGround(player_z: fixed, player_floor_z: fixed) -> bool
{
	player_z <= player_floor_z
}

#[unsafe(no_mangle)]
pub extern "C" fn MovePlayer(
	player: *mut Player,
	on_ground: bool)
{
	unsafe {
		let player = &mut *player;

		player.move_player(on_ground);
	}
}