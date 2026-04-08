//
// C-to-Rust interface table for Doom Rust modules.
//

#include "rust_interface.h"

#include "p_local.h"
#include "p_spec.h"
#include "s_sound.h"
#include "z_zone.h"

void D_InitRustInterface(void)
{
    INTERFACE.P_TeleportByLineTag = P_TeleportByLineTag;
    INTERFACE.P_TeleportMove = &P_TeleportMove;
    INTERFACE.S_StartSound = S_StartSound;
    INTERFACE.P_PlayerThink = P_PlayerThink;
    INTERFACE.P_MovePsprites = P_MovePsprites;
    INTERFACE.P_CalcHeight = P_CalcHeight;
    INTERFACE.P_UpdateSpecials = P_UpdateSpecials;
    INTERFACE.P_RespawnSpecials = P_RespawnSpecials;
    INTERFACE.P_SetMobjState = P_SetMobjState;
    INTERFACE.P_GetMobjState = P_GetMobjState;
    INTERFACE.P_SpawnMobj = P_SpawnMobj;
    INTERFACE.Z_Free = Z_Free;
}
