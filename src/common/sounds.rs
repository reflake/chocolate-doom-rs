pub const NORM_PITCH: i32 = 127;

#[repr(C)]
pub struct SfxInfo {
    // tag name, used for hexen.
	pub tag_name: *const std::ffi::c_uchar,

    // lump name.  If we are running with use_sfx_prefix=true, a
    // 'DS' (or 'DP' for PC speaker sounds) is prepended to this.
    pub name: [std::ffi::c_uchar; 12],

    // Sfx priority
    pub priority: i32,

    // referenced sound if a link
    pub link: *const SfxInfo,

    // pitch if a link (Doom), whether to pitch-shift (Hexen)
    pub pitch: i32,

    // volume if a link
    pub volume: i32,

    // this is checked every second to see if sound
    // can be thrown out (if 0, then decrement, if -1,
    // then throw out, if > 0, then it is in use)
    pub usefulness: i32,

    // lump number of sfx
    pub lumpnum: i32,

    // Maximum number of channels that the sound can be played on 
    // (Heretic)
    pub numchannels: i32,

    // data used by the low level code
    pub driver_data: *mut std::ffi::c_void,
}