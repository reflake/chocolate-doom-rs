#include "d_think.h"
#include "doomtype.h"
#include "i_system.h"
#include "p_local.h"
#include "p_mobj.h"
#include <stdio.h>
#include "p_tick.h"

extern thinker_t thinkercap;

enum DumpMode : int {
	DM_Mobjs = 0x01,

	DM_Checksum = 0x20,
};

extern enum DumpMode dumpMode;
extern FILE*         dumpFile;
extern int           targetTick;

int CalculateCrc(const unsigned char* buf, size_t len, int state);

void DumpState();

void SaveDumped();