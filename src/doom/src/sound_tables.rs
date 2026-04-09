

//
// Information about all the sfx
//
use common::sounds::SfxInfo;

use crate::sounds::SfxEnum;

#[allow(static_mut_refs)]
pub fn get_sfx_info(sfx_id: SfxEnum) -> Option<&'static mut SfxInfo> {
	unsafe {
		// check for bogus sound #
		if (1..S_sfx.len()).contains(&(sfx_id as usize))
		{
			return Some(&mut S_sfx[sfx_id as usize])
		}

		None
	}
}

const fn sound(name: &'static str, priority: i32) -> SfxInfo {

	// agh this is so bullshit!
	let mut sfx_name = [0u8; 12];
	let mut i = 0;

	while i < name.len() {
		sfx_name[i] = name.as_bytes()[i];

		i += 1;
	}

	SfxInfo {
		tag_name: std::ptr::null(),
		name: sfx_name,
		priority,
		link: std::ptr::null(),
		pitch: -1,
		volume: -1,
		usefulness: 0,
		lumpnum: 0,
		numchannels: -1,
		driver_data: std::ptr::null_mut(),
	}
}

const fn sound_link(name: &'static str, priority: i32, link: SfxEnum, pitch: i32, volume: i32) -> SfxInfo {

	let mut sfx_name = [0u8; 12];
	let mut i = 0;

	while i < name.len() {
		sfx_name[i] = name.as_bytes()[i];

		i += 1;
	}

	SfxInfo {
		tag_name: std::ptr::null(),
		name: sfx_name,
		priority,
		link: unsafe { &S_sfx[link as usize] },
		pitch,
		volume,
		usefulness: 0,
		lumpnum: 0,
		numchannels: -1,
		driver_data: std::ptr::null_mut(),
	}
}

#[unsafe(no_mangle)]
static mut S_sfx: [SfxInfo; 109] =
[
  // S_sfx[0] needs to be a dummy for odd reasons.
  sound("none",   0),
  sound("pistol", 64),
  sound("shotgn", 64),
  sound("sgcock", 64),
  sound("dshtgn", 64),
  sound("dbopn",  64),
  sound("dbcls",  64),
  sound("dbload", 64),
  sound("plasma", 64),
  sound("bfg",    64),
  sound("sawup",  64),
  sound("sawidl", 118),
  sound("sawful", 64),
  sound("sawhit", 64),
  sound("rlaunc", 64),
  sound("rxplod", 70),
  sound("firsht", 70),
  sound("firxpl", 70),
  sound("pstart", 100),
  sound("pstop",  100),
  sound("doropn", 100),
  sound("dorcls", 100),
  sound("stnmov", 119),
  sound("swtchn", 78),
  sound("swtchx", 78),
  sound("plpain", 96),
  sound("dmpain", 96),
  sound("popain", 96),
  sound("vipain", 96),
  sound("mnpain", 96),
  sound("pepain", 96),
  sound("slop",   78),
  sound("itemup", 78),
  sound("wpnup",  78),
  sound("oof",    96),
  sound("telept", 32),
  sound("posit1", 98),
  sound("posit2", 98),
  sound("posit3", 98),
  sound("bgsit1", 98),
  sound("bgsit2", 98),
  sound("sgtsit", 98),
  sound("cacsit", 98),
  sound("brssit", 94),
  sound("cybsit", 92),
  sound("spisit", 90),
  sound("bspsit", 90),
  sound("kntsit", 90),
  sound("vilsit", 90),
  sound("mansit", 90),
  sound("pesit",  90),
  sound("sklatk", 70),
  sound("sgtatk", 70),
  sound("skepch", 70),
  sound("vilatk", 70),
  sound("claw",   70),
  sound("skeswg", 70),
  sound("pldeth", 32),
  sound("pdiehi", 32),
  sound("podth1", 70),
  sound("podth2", 70),
  sound("podth3", 70),
  sound("bgdth1", 70),
  sound("bgdth2", 70),
  sound("sgtdth", 70),
  sound("cacdth", 70),
  sound("skldth", 70),
  sound("brsdth", 32),
  sound("cybdth", 32),
  sound("spidth", 32),
  sound("bspdth", 32),
  sound("vildth", 32),
  sound("kntdth", 32),
  sound("pedth",  32),
  sound("skedth", 32),
  sound("posact", 120),
  sound("bgact",  120),
  sound("dmact",  120),
  sound("bspact", 100),
  sound("bspwlk", 100),
  sound("vilact", 100),
  sound("noway",  78),
  sound("barexp", 60),
  sound("punch",  64),
  sound("hoof",   70),
  sound("metal",  70),
  sound_link("chgun", 64, SfxEnum::sfx_pistol, 150, 0),
  sound("tink",   60),
  sound("bdopn",  100),
  sound("bdcls",  100),
  sound("itmbk",  100),
  sound("flame",  32),
  sound("flamst", 32),
  sound("getpow", 60),
  sound("bospit", 70),
  sound("boscub", 70),
  sound("bossit", 70),
  sound("bospn",  70),
  sound("bosdth", 70),
  sound("manatk", 70),
  sound("mandth", 70),
  sound("sssit",  70),
  sound("ssdth",  70),
  sound("keenpn", 70),
  sound("keendt", 70),
  sound("skeact", 70),
  sound("skesit", 70),
  sound("skeatk", 70),
  sound("radio",  60),
];