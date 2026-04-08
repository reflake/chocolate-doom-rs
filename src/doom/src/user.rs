use common::fixed::fixed;
use common::trigonometry::{ang};

use crate::info::StateEnum;
use crate::mobj::onground;
use crate::player::Player;

//
// DESCRIPTION:
//	Player related stuff.
//	Bobbing POV/weapon, movement.
//	Pending weapon.
//

impl Player {
	pub fn move_player(&mut self) {

		let mobj = self.mobj().unwrap();

		mobj.angle += ang::from_hi(self.cmd.angle_turn);

		if self.on_ground() {
			mobj.thrust(mobj.angle, fixed(self.cmd.forward_move as i32 * 2048));
			mobj.thrust(mobj.angle - ang::degree(90.0), fixed(self.cmd.side_move as i32 * 2048));
		}

		if self.cmd.moving() 
			&& mobj.get_state() == StateEnum::S_PLAY
		{
			mobj.set_state(StateEnum::S_PLAY_RUN1);
		}
	}

	pub fn on_ground(&self) -> bool {
		let mobj = self.mobj().unwrap();

		mobj.position.z <= mobj.floorz
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn PlayerOnGround(player_z: fixed, player_floor_z: fixed) -> bool
{
	player_z <= player_floor_z
}

#[unsafe(no_mangle)]
pub extern "C" fn P_MovePlayer(player: *mut Player)
{
	unsafe {
		let player = &mut *player;
		onground = player.on_ground();

		player.move_player();
	}
}