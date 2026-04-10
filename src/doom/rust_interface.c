//
// C-to-Rust interface table for Doom Rust modules.
//

#include "rust_interface.h"

#include "i_sound.h"
#include "i_system.h"
#include "p_local.h"
#include "p_spec.h"
#include "z_zone.h"

void I_ExError(const char *error) {
	I_Error("%s", error);
}

void D_InitRustInterface(void)
{
    INTERFACE.P_TeleportByLineTag = P_TeleportByLineTag;
    INTERFACE.P_TeleportMove = P_TeleportMove;
    INTERFACE.P_PlayerThink = P_PlayerThink;
    INTERFACE.P_MovePsprites = P_MovePsprites;
    INTERFACE.P_UpdateSpecials = P_UpdateSpecials;
    INTERFACE.P_RespawnSpecials = P_RespawnSpecials;
    INTERFACE.P_SetMobjState = P_SetMobjState;
    INTERFACE.P_GetMobjState = P_GetMobjState;
    INTERFACE.P_SpawnMobj = P_SpawnMobj;
	INTERFACE.P_SpawnMissile = P_SpawnMissile;

    INTERFACE.Z_Free = Z_Free;
	INTERFACE.I_Error = I_ExError;
	INTERFACE.I_StartSound = I_StartSound;
	INTERFACE.I_GetSfxLumpNum = I_GetSfxLumpNum;
	INTERFACE.I_StopSound = I_StopSound;
	INTERFACE.I_SoundIsPlaying = I_SoundIsPlaying;
	INTERFACE.I_UpdateSounds = I_UpdateSound;
	INTERFACE.I_UpdateSoundParams = I_UpdateSoundParams;
}
