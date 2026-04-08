#[allow(nonstandard_style, dead_code)]
#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameVersion
{
    exe_doom_1_2,    // Doom 1.2: shareware and registered
    exe_doom_1_5,    // Doom 1.5: "
    exe_doom_1_666,  // Doom 1.666: for shareware, registered and commercial
    exe_doom_1_7,    // Doom 1.7/1.7a: "
    exe_doom_1_8,    // Doom 1.8: "
    exe_doom_1_9,    // Doom 1.9: "
    exe_hacx,        // Hacx
    exe_ultimate,    // Ultimate Doom (retail)
    exe_final,       // Final Doom
    exe_final2,      // Final Doom (alternate exe)
    exe_chex,        // Chex Quest executable (based on Final Doom)

    exe_heretic_1_3, // Heretic 1.3

    exe_hexen_1_1,   // Hexen 1.1
    exe_hexen_1_1r2, // Hexen 1.1 (alternate exe)
    exe_strife_1_2,  // Strife v1.2
    exe_strife_1_31  // Strife v1.31
}

// Skill level.

#[allow(nonstandard_style, dead_code)]
#[repr(i32)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SkillLevel
{
    sk_noitems = -1,        // the "-skill 0" hack
    sk_baby = 0,
    sk_easy,
    sk_medium,
    sk_hard,
    sk_nightmare
}