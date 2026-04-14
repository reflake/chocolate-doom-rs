#include "dumping.h"
#include "doomdata.h"
#include "doomstat.h"
#include "i_system.h"
#include "info.h"
#include "m_fixed.h"
#include "p_mobj.h"
#include "strife/hu_stuff.h"
#include "z_zone.h"
#include <stdio.h>
#include <stdlib.h>

enum DumpMode dumpMode = 0;
FILE*         dumpFile = NULL;
int           targetTick = -1;
int           currentTick = 0;

struct state_t_st {
	spritenum_t sprite;
	int frame;
	int tics;
	// void (*action) ();
	actionf_t action;
	statenum_t nextstate;
	int misc1;
	int misc2;
};

struct state_t_st CastStateToDumpState(state_t* state)
{
	struct state_t_st s;

	s.sprite = state->sprite;
	s.frame = state->frame;
	s.tics = state->tics;
	s.action = state->action;
	s.nextstate = state->nextstate;
	s.misc1 = state->misc1;
	s.misc2 = state->misc2;

	return s;
}

struct mobj_t_st {
	fixed_t x, y, z;
	angle_t angle;
	spritenum_t sprite;
	int frame;
	fixed_t floorz, ceilingz;
	fixed_t radius, height;
	fixed_t momx, momy, momz;
	int validcount;
	mobjtype_t type;
	int tics;
	statenum_t state;
	int flags;
	int health;
	int movedir;
	int movecount;
	int target_player;
	int reactiontime;
	int threshold;
	int lastlook;
	mapthing_t spawnpoint;
};

struct mobj_t_st CastMobjToDumpState(mobj_t* mobj)
{
	struct mobj_t_st state;

	state.x = mobj->x;
	state.y = mobj->y;
	state.z = mobj->z;
	state.angle = mobj->angle;
	state.sprite = mobj->sprite;
	state.frame = mobj->frame;
	state.floorz = mobj->floorz;
	state.ceilingz = mobj->ceilingz;
	state.radius = mobj->radius;
	state.height = mobj->height;
	state.momx = mobj->momx;
	state.momy = mobj->momy;
	state.momz = mobj->momz;
	state.validcount = mobj->validcount;
	state.type = mobj->type;
	state.tics = mobj->tics;
	state.state = P_GetMobjState(mobj);
	state.flags = mobj->flags;
	state.health = mobj->health;
	state.movedir = mobj->movedir;
	state.movecount = mobj->movecount;
	state.target_player = -1;
	state.reactiontime = mobj->reactiontime;
	state.threshold = mobj->threshold;
	state.lastlook = mobj->lastlook;

	if (mobj->target != NULL)
	{
		for(int i = 0; i < MAXPLAYERS; i++)
		{
			if (players[i].mo == mobj->target)
			{
				state.target_player = i;
				break;
			}
		}
	}

	// spawnpoint is not stored in the original struct, so we will just set it to 0
	memcpy(&state.spawnpoint, &(mobj->spawnpoint), sizeof(mapthing_t));

	return state;
}


void DumpMobjs(int crc);

void DumpState()
{
	currentTick++;

	if (dumpFile == NULL) return;
	if (targetTick != -1 && currentTick != targetTick) return;

	fprintf(dumpFile, "frame: %i\n", currentTick);

	DumpMobjs(1);

	fprintf(dumpFile, "\n");

	
	if (currentTick == targetTick)
	{
		I_Quit();
	}
}

const char player_names_with_spaces_arr [MAXPLAYERS][16] = {
	" (Player 1)",
	" (Player 2)",
	" (Player 3)",
	" (Player 4)"
};

const char player_names_arr[MAXPLAYERS][16] = {
	"Player 1",
	"Player 2",
	"Player 3",
	"Player 4"
};

void DumpMobjs(int crc)
{
	if (!(dumpMode & DM_Mobjs)) return;

	boolean checksumMode = dumpMode & DM_Checksum;

	for(thinker_t* t = thinkercap.next; t != &thinkercap; t = t->next)
	{
		if (t->function.acp1 == (actionf_p1)P_MobjThinker)
		{
			int playerIndex = -1;
			const char* targetname = "no target";

			struct mobj_t_st   mobj = CastMobjToDumpState((mobj_t*)t);

			for(int i = 0; i < MAXPLAYERS; i++)
			{
				if (players[i].mo == (mobj_t*)t)
				{
					playerIndex = i;
					break;
				}
			}

			if (mobj.target_player != -1)
			{
				targetname = player_names_arr[mobj.target_player];
			}

			if (checksumMode)
				crc = CalculateCrc((unsigned char*)&mobj, sizeof(struct mobj_t_st), crc);
			else {
				fprintf(dumpFile, "  mobj%s: x=%i y=%i z=%i angle=%i sprite=%i frame=%i floorz=%i ceilingz=%i radius=%i height=%i momx=%i momy=%i momz=%i validcount=%i type=%i tics=%i state=%i flags=%i health=%i movedir=%i movecount=%i target=%s reactiontime=%i threshold=%i lastlook=%i spawnpoint={x=%i y=%i angle=%i type=%i options=%i}\n",
					playerIndex != -1 ? player_names_with_spaces_arr[playerIndex] : "",
					mobj.x, mobj.y, mobj.z, mobj.angle, mobj.sprite, mobj.frame, mobj.floorz, mobj.ceilingz, mobj.radius, mobj.height, mobj.momx, mobj.momy, mobj.momz, mobj.validcount, mobj.type, mobj.tics, mobj.state,
					mobj.flags, mobj.health, mobj.movedir, mobj.movecount, targetname, mobj.reactiontime, mobj.threshold, mobj.lastlook,
					mobj.spawnpoint.x, mobj.spawnpoint.y, mobj.spawnpoint.angle, mobj.spawnpoint.type, mobj.spawnpoint.options);
			}


		}
	}
	
	if (checksumMode)
		fprintf(dumpFile, "  mobjs crc: %08x\n", crc);
}



int CalculateCrc(const unsigned char* buf, size_t len, int state) {
	unsigned int sL = state & 0xFFFF;
	unsigned int sH = (state >> 16) & 0xFFFF;

	for(int i = 0; i < len; i++) {
		sL = (sL + buf[i]) % 65521;
		sH = (sH + sL) % 65521;
	}
	return (sH << 16) | (sL & 0xFFFF);
}

void SaveDumped()
{
	if (dumpFile)
	{
		fclose(dumpFile);
		dumpFile = NULL;
	}
}