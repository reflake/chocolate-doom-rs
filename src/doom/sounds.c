//
// Copyright(C) 1993-1996 Id Software, Inc.
// Copyright(C) 2005-2014 Simon Howard
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// DESCRIPTION:
//	Created by a sound utility.
//	Kept as a sample, DOOM2 sounds.
//


#include <stdlib.h>


#include "doomtype.h"
#include "sounds.h"

//
// Information about all the music
//

#define MUSIC(name) \
    { name, 0, NULL, NULL }

musicinfo_t S_music[] =
{
    MUSIC(NULL),
    MUSIC("e1m1"),
    MUSIC("e1m2"),
    MUSIC("e1m3"),
    MUSIC("e1m4"),
    MUSIC("e1m5"),
    MUSIC("e1m6"),
    MUSIC("e1m7"),
    MUSIC("e1m8"),
    MUSIC("e1m9"),
    MUSIC("e2m1"),
    MUSIC("e2m2"),
    MUSIC("e2m3"),
    MUSIC("e2m4"),
    MUSIC("e2m5"),
    MUSIC("e2m6"),
    MUSIC("e2m7"),
    MUSIC("e2m8"),
    MUSIC("e2m9"),
    MUSIC("e3m1"),
    MUSIC("e3m2"),
    MUSIC("e3m3"),
    MUSIC("e3m4"),
    MUSIC("e3m5"),
    MUSIC("e3m6"),
    MUSIC("e3m7"),
    MUSIC("e3m8"),
    MUSIC("e3m9"),
    MUSIC("inter"),
    MUSIC("intro"),
    MUSIC("bunny"),
    MUSIC("victor"),
    MUSIC("introa"),
    MUSIC("runnin"),
    MUSIC("stalks"),
    MUSIC("countd"),
    MUSIC("betwee"),
    MUSIC("doom"),
    MUSIC("the_da"),
    MUSIC("shawn"),
    MUSIC("ddtblu"),
    MUSIC("in_cit"),
    MUSIC("dead"),
    MUSIC("stlks2"),
    MUSIC("theda2"),
    MUSIC("doom2"),
    MUSIC("ddtbl2"),
    MUSIC("runni2"),
    MUSIC("dead2"),
    MUSIC("stlks3"),
    MUSIC("romero"),
    MUSIC("shawn2"),
    MUSIC("messag"),
    MUSIC("count2"),
    MUSIC("ddtbl3"),
    MUSIC("ampie"),
    MUSIC("theda3"),
    MUSIC("adrian"),
    MUSIC("messg2"),
    MUSIC("romer2"),
    MUSIC("tense"),
    MUSIC("shawn3"),
    MUSIC("openin"),
    MUSIC("evil"),
    MUSIC("ultima"),
    MUSIC("read_m"),
    MUSIC("dm2ttl"),
    MUSIC("dm2int") 
};


//
// Information about all the sfx
//

#define SOUND(name, priority) \
  { NULL, name, priority, NULL, -1, -1, 0, 0, -1, NULL }
#define SOUND_LINK(name, priority, link_id, pitch, volume) \
  { NULL, name, priority, &S_sfx[link_id], pitch, volume, 0, 0, -1, NULL }
