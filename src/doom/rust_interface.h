//
// C-to-Rust interface table for Doom Rust modules.
//

#ifndef __DOOM_RUST_INTERFACE__
#define __DOOM_RUST_INTERFACE__

#include "doomtype.h"
#include "d_player.h"
#include "m_fixed.h"
#include "p_mobj.h"
#include "r_defs.h"

// Declared in p_mobj.c but not exposed by public headers.
void *P_TeleportByLineTag(line_t *line);
void P_PlayerThink(player_t *player);
void I_ExError(const char *error);
statenum_t P_GetMobjState(mobj_t *mobj);

typedef struct
{
    void *(*P_TeleportByLineTag)(line_t *line);
    boolean (*P_TeleportMove)(mobj_t *thing, fixed_t x, fixed_t y);
    void (*S_StartSound)(void *origin_p, int sfx_id);
    void (*P_PlayerThink)(player_t *player);
    void (*P_MovePsprites)(player_t *player);
    void (*P_UpdateSpecials)(void);
    void (*P_RespawnSpecials)(void);
    boolean (*P_SetMobjState)(mobj_t *mobj, statenum_t state);
    statenum_t (*P_GetMobjState)(mobj_t *mobj);
    mobj_t *(*P_SpawnMobj)(fixed_t x, fixed_t y, fixed_t z, mobjtype_t obj_type);
    void (*Z_Free)(void *void_ptr);
	void (*I_Error)(const char *error);
} rust_interface_t;

extern rust_interface_t INTERFACE;

void D_InitRustInterface(void);

#endif
