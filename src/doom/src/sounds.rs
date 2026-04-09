use common::{bool::{FALSE, TRUE}, fixed::fixed, sounds::{NORM_PITCH, SfxInfo}, trigonometry::{R_PointToAngle2, ang}};

use crate::{I_Error, external::INTERFACE, mobj::Mobj, player::Player, random::random, sound_tables, stat::gamemap};

#[unsafe(no_mangle)]
pub static mut SOUND_ENGINE: SoundEngine = SoundEngine { 
	global_volume: 0,
	channels: Vec::new(),
};
pub struct Channel {
    // sound information (if null, channel avail.)
    sfx: *mut SfxInfo,

    // origin of sound
    origin: *mut Mobj,

    // handle of the sound being played
    handle: i32,

    pitch: i32,

	index: i32,
}

#[repr(C)]
pub struct SoundEngine {
	// Internal volume level, ranging from 0-127
	global_volume: i32,

	// The set of channels available
	channels: Vec<Channel>,
}

enum AdjustedParameters {
	Inaudible,
	Parameters { separation: i32, volume: i32 }
}

const NORM_SEPARATION: i32 = 128;

impl Channel {
	#[allow(static_mut_refs)]
	pub fn stop(&mut self) {

		if self.sfx.is_null() { return; }

		// stop the sound playing
		unsafe {
			if INTERFACE.I_SoundIsPlaying(self.handle) == TRUE
			{
				INTERFACE.I_StopSound(self.handle);
			}
		}

		// check to see if other channels are playing the sound

		/*for (i=0; i<snd_channels; i++)
		{
			if (cnum != i && c->sfxinfo == channels[i].sfxinfo)
			{
				break;
			}
		}*/

		// degrade usefulness of sound data

		unsafe {
			(&mut *self.sfx).usefulness -= 1; // check correctnes of dereferencing
		}
		self.sfx = std::ptr::null_mut();
		self.origin = std::ptr::null_mut();
	}
}

impl SoundEngine {

	#[allow(static_mut_refs)]
	pub fn emit_sound(&mut self, source: Option<&Mobj>, sfx_id: SfxEnum) {

		// check for bogus sound #
		let Some(sfx) = sound_tables::get_sfx_info(sfx_id) else {
			
			return I_Error!("Bad sfx #: {}", sfx_id as u32);
		};
		
		// Initialize sound parameters
		let mut pitch = NORM_PITCH;
		let mut vol = self.global_volume;

		if sfx.link != std::ptr::null() {
			vol    += sfx.volume;
			pitch  += sfx.pitch;
		
			if vol < 1 {
				return;
			}

			if vol > self.global_volume {
				vol = self.global_volume;
			}
		}

		let mut sep = NORM_SEPARATION;

		// Check to see if it is audible,
		//  and if not, modify the params
		if let Some(listener) = Player::local_player().mobj() 
			&& let Some(source) = source
			&& !std::ptr::eq(listener, source)
		{
			match self.adjust_sound_params(listener, source) {
				AdjustedParameters::Inaudible => return,
				AdjustedParameters::Parameters { separation, volume } => {

					if source.position == listener.position {
						sep = NORM_SEPARATION;
					} else {
						sep = separation;
					}

					vol = volume;
				}
			}
		}

		// hacks to vary the sfx pitches
		unsafe {
			if sfx_id >= SfxEnum::sfx_sawup && sfx_id <= SfxEnum::sfx_sawhit 
			{
				pitch += 8 - (random.next() & 15);
			}
			else if sfx_id != SfxEnum::sfx_itemup && sfx_id != SfxEnum::sfx_tink 
			{
				pitch += 16 - (random.next() & 31);
			}
		}

		pitch = pitch.clamp(0, 255);

		// kill old sound
		self.stop_sound(source);
		
		// try to find a channel
		let chan = self.get_channel(source, sfx_id);

		if chan.is_none() {
			// no channel available
			return;
		}

		let chan = chan.unwrap();

		// increase the usefulness of the sound's SfxInfo
		sfx.usefulness += 1;

		if sfx.usefulness < 0 {
			sfx.usefulness = 1;
		}

		// is this some kind of precaching?
		if sfx.lumpnum < 0 {
			sfx.lumpnum = unsafe { INTERFACE.I_GetSfxLumpNum(sfx) };
		}

		chan.pitch  = pitch;
		chan.handle = unsafe { INTERFACE.I_StartSound(sfx, chan.index, vol, sep, pitch) };
	}

	pub fn stop_sound(&mut self, origin: Option<&Mobj>) {
		for ch in &mut self.channels {
			if !ch.sfx.is_null()
			   && std::ptr::eq(ch.origin, unsafe { std::mem::transmute(origin) })
			{
				ch.stop();
			}
		}
	}

	//
	// Changes volume and stereo-separation variables
	//  from the norm of a sound effect to be played.
	// If the sound is not audible, returns a 0.
	// Otherwise, modifies parameters and returns 1.
	//
	fn adjust_sound_params(&self, listener: &Mobj, source: &Mobj) -> AdjustedParameters {
		
		unsafe {
			// calculate the distance to sound origin
			//  and clip it if necessary
			let mut offset = source.position - listener.position;
			offset.x = offset.x.abs();
			offset.y = offset.y.abs();

			// From _GG1_ p.428. Appox. eucledian distance fast.
			let approx_dist = offset.x + offset.y - (common::fixed::min(offset.x, offset.y) / 2);

			const CLIPPING_DISTANCE: fixed = fixed::from_int(1200);

			if gamemap != 8 && approx_dist > CLIPPING_DISTANCE {
				// too far away, no sound
				return AdjustedParameters::Inaudible;
			}

			// angle of source to listener
			let angle = R_PointToAngle2(listener.position.xy(), source.position.xy());

			let angle = if angle > listener.angle {
				angle - listener.angle
			} else {
				angle + ang(0xFFFFFFFF - listener.angle.0)
			};

			const STEREO_SWING: fixed = fixed::from_int(96);

    		// stereo separation
			let separation: i32 = (fixed::from_int(128) - STEREO_SWING * angle.fine_sine()).into();

			const CLOSE_DISTANCE: fixed = fixed::from_int(200);
			let   attenuator:     fixed = CLIPPING_DISTANCE - CLOSE_DISTANCE;

    		// volume calculation
			let volume: i32 = if approx_dist < CLOSE_DISTANCE {
				self.global_volume
			} else if gamemap == 8 {
				let approx_dist = common::fixed::min(approx_dist, CLIPPING_DISTANCE);

				15 + ((CLIPPING_DISTANCE - approx_dist) / attenuator * (self.global_volume - 15)).to_int()
			} else {
				((CLIPPING_DISTANCE - approx_dist) / attenuator * self.global_volume).to_int()
			};

			assert!(volume <= 127);

			match volume > 0 {
				true  => AdjustedParameters::Parameters { separation, volume },
				false => AdjustedParameters::Inaudible,
			}
		}
	}

	// rename to new channel
	fn get_channel(&mut self, origin: Option<&Mobj>, sfx: SfxEnum) -> Option<&mut Channel> {
		
		// Find an open channel
		if let Some(open_chan_idx) = self.channels.iter().position(|ch| ch.sfx.is_null()) {
    		// channel is decided to be cnum.
			let open_chan = &mut self.channels[open_chan_idx];
			open_chan.stop();
			open_chan.sfx    = sound_tables::get_sfx_info(sfx).unwrap();
			open_chan.origin = unsafe { std::mem::transmute(origin) };

			return Some(open_chan);
		}

    	// None available
        // Look for lower priority
		let Some(sfx_info) = sound_tables::get_sfx_info(sfx) else {
			return None;
		};

		let priority = sfx_info.priority;

		unsafe {
			if let Some(lp_chan) = self.channels.iter_mut().find(|ch| ch.sfx.is_null() 
																	|| ch.sfx.as_ref().unwrap().priority < priority) {
				// Kick out lower priority.
				lp_chan.stop();
				lp_chan.sfx    = sound_tables::get_sfx_info(sfx).unwrap();
				lp_chan.origin = std::mem::transmute(origin);

				return Some(lp_chan);
			}
		}

        // FUCK!  No lower priority.  Sorry, Charlie.
		None
	}

	//
	// Updates music & sounds
	//

	#[allow(static_mut_refs)]
	fn update_sounds(&mut self, listener: Option<&Mobj>) {
		unsafe { 

			INTERFACE.I_UpdateSounds();

			for i in 0..self.channels.len() {
				let ch = &self.channels[i];
				if !ch.sfx.is_null() {
					if INTERFACE.I_SoundIsPlaying(ch.handle) == FALSE {
						// if channel is allocated but sound has stopped,
						//  free it
						(&mut self.channels[i]).stop();
						continue;
					}

					if let Some(sfx_info) = ch.sfx.as_ref() {
						if sfx_info.link != std::ptr::null() {
							let volume = sfx_info.volume + self.global_volume;

							if volume < 1 {
								(&mut self.channels[i]).stop();
								continue;
							}
						}
					}

					if let Some(origin) = ch.origin.as_ref()
						&& let Some(listener) = listener
						&& !std::ptr::eq(listener, origin)
					{
						match self.adjust_sound_params(listener, origin) {
							AdjustedParameters::Inaudible => (&mut self.channels[i]).stop(),
							AdjustedParameters::Parameters { separation, volume } =>
								INTERFACE.I_UpdateSoundParams(ch.handle, volume, separation),
						}
					}
				}
			}
		}
	}
}

impl Mobj {
	#[allow(static_mut_refs)]
	pub fn emit_sound(&self, sfx_id: SfxEnum) {
		unsafe {
			SOUND_ENGINE.emit_sound(Some(self), sfx_id);
		}
	}
}

#[allow(static_mut_refs)]
pub fn st_emit_sound(sfx_id: SfxEnum) {
	unsafe {
		SOUND_ENGINE.emit_sound(None, sfx_id);
	}
}

#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
extern "C" fn S_UpdateSounds(listener: *const Mobj) {
	unsafe {
		SOUND_ENGINE.update_sounds(listener.as_ref());
	}
}

#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
extern "C" fn S_StartSound(origin: *const Mobj, sfx_id: SfxEnum) {
	unsafe {
		SOUND_ENGINE.emit_sound(origin.as_ref(), sfx_id);
	}
}

#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
extern "C" fn S_StopSound(origin: *const Mobj) {
	unsafe {
		SOUND_ENGINE.stop_sound(origin.as_ref());
	}
}

#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
extern "C" fn StopAllSounds() {
	unsafe {
		for ch in &mut SOUND_ENGINE.channels {
			ch.stop();
		}
	}
}

#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
extern "C" fn AllocChannels(num_channels: i32) {
	unsafe {
		// channels = Z_Malloc(snd_channels*sizeof(channel_t), PU_STATIC, 0);

		(&mut SOUND_ENGINE).channels = (0..num_channels)
			.map(|i| Channel {
				sfx: std::ptr::null_mut(),
				origin: std::ptr::null_mut(),
				handle: 0,
				pitch: 0,
				index: i,
			})
			.collect();
	}
}

#[allow(nonstandard_style, dead_code)]
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SfxEnum
{
    sfx_None,
    sfx_pistol,
    sfx_shotgn,
    sfx_sgcock,
    sfx_dshtgn,
    sfx_dbopn,
    sfx_dbcls,
    sfx_dbload,
    sfx_plasma,
    sfx_bfg,
    sfx_sawup,
    sfx_sawidl,
    sfx_sawful,
    sfx_sawhit,
    sfx_rlaunc,
    sfx_rxplod,
    sfx_firsht,
    sfx_firxpl,
    sfx_pstart,
    sfx_pstop,
    sfx_doropn,
    sfx_dorcls,
    sfx_stnmov,
    sfx_swtchn,
    sfx_swtchx,
    sfx_plpain,
    sfx_dmpain,
    sfx_popain,
    sfx_vipain,
    sfx_mnpain,
    sfx_pepain,
    sfx_slop,
    sfx_itemup,
    sfx_wpnup,
    sfx_oof,
    sfx_telept,
    sfx_posit1,
    sfx_posit2,
    sfx_posit3,
    sfx_bgsit1,
    sfx_bgsit2,
    sfx_sgtsit,
    sfx_cacsit,
    sfx_brssit,
    sfx_cybsit,
    sfx_spisit,
    sfx_bspsit,
    sfx_kntsit,
    sfx_vilsit,
    sfx_mansit,
    sfx_pesit,
    sfx_sklatk,
    sfx_sgtatk,
    sfx_skepch,
    sfx_vilatk,
    sfx_claw,
    sfx_skeswg,
    sfx_pldeth,
    sfx_pdiehi,
    sfx_podth1,
    sfx_podth2,
    sfx_podth3,
    sfx_bgdth1,
    sfx_bgdth2,
    sfx_sgtdth,
    sfx_cacdth,
    sfx_skldth,
    sfx_brsdth,
    sfx_cybdth,
    sfx_spidth,
    sfx_bspdth,
    sfx_vildth,
    sfx_kntdth,
    sfx_pedth,
    sfx_skedth,
    sfx_posact,
    sfx_bgact,
    sfx_dmact,
    sfx_bspact,
    sfx_bspwlk,
    sfx_vilact,
    sfx_noway,
    sfx_barexp,
    sfx_punch,
    sfx_hoof,
    sfx_metal,
    sfx_chgun,
    sfx_tink,
    sfx_bdopn,
    sfx_bdcls,
    sfx_itmbk,
    sfx_flame,
    sfx_flamst,
    sfx_getpow,
    sfx_bospit,
    sfx_boscub,
    sfx_bossit,
    sfx_bospn,
    sfx_bosdth,
    sfx_manatk,
    sfx_mandth,
    sfx_sssit,
    sfx_ssdth,
    sfx_keenpn,
    sfx_keendt,
    sfx_skeact,
    sfx_skesit,
    sfx_skeatk,
    sfx_radio,
    NUMSFX
}