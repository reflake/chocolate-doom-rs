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
// DESCRIPTION:  none
//

#include <stdio.h>
#include <stdlib.h>

#include "i_sound.h"
#include "i_system.h"

#include "deh_str.h"

#include "doomstat.h"
#include "doomtype.h"

#include "sounds.h"
#include "s_sound.h"

#include "m_misc.h"
#include "m_random.h"
#include "m_argv.h"

#include "p_local.h"
#include "w_wad.h"
#include "z_zone.h"

// when to clip out sounds
// Does not fit the large outdoor areas.

#define S_CLIPPING_DIST (1200 * FRACUNIT)

// Distance tp origin when sounds should be maxed out.
// This should relate to movement clipping resolution
// (see BLOCKMAP handling).
// In the source code release: (160*FRACUNIT).  Changed back to the
// Vanilla value of 200 (why was this changed?)

#define S_CLOSE_DIST (200 * FRACUNIT)

// The range over which sound attenuates

#define S_ATTENUATOR ((S_CLIPPING_DIST - S_CLOSE_DIST) >> FRACBITS)

// Stereo separation

#define S_STEREO_SWING (96 * FRACUNIT)

#define NORM_PRIORITY 64
#define NORM_SEP 128

// Maximum volume of a sound effect.
// Internal default is max out of 0-15.

int sfxVolume = 8;

// Maximum volume of music.

int musicVolume = 8;

// Whether songs are mus_paused

static boolean mus_paused;

// Music currently being played

static musicinfo_t *mus_playing = NULL;

// Number of channels to use

int snd_channels = 8;

extern void AllocChannels(int numchannels);
//
// Initializes sound stuff, including volume
// Sets channels, SFX and music volume,
//  allocates channel buffer, sets S_sfx lookup.
//


void S_Init(int sfxVolume, int musicVolume)
{
    int i;

    if (gameversion == exe_doom_1_666)
    {
        if (logical_gamemission == doom)
        {
            I_SetOPLDriverVer(opl_doom1_1_666);
        }
        else
        {
            I_SetOPLDriverVer(opl_doom2_1_666);
        }
    }
    else
    {
        I_SetOPLDriverVer(opl_doom_1_9);
    }

    I_PrecacheSounds(S_sfx, NUMSFX);

    S_SetSfxVolume(sfxVolume);
    S_SetMusicVolume(musicVolume);

    // Allocating the internal channels for mixing
    // (the maximum numer of sounds rendered
    // simultaneously) within zone memory.
	AllocChannels(snd_channels);

    // no sounds are playing, and they are not mus_paused
    mus_paused = 0;

    // Note that sounds have not been cached (yet).
    for (i=1 ; i<NUMSFX ; i++)
    {
        S_sfx[i].lumpnum = S_sfx[i].usefulness = -1;
    }

    // Doom defaults to pitch-shifting off.
    if (snd_pitchshift == -1)
    {
        snd_pitchshift = 0;
    }

    I_AtExit(S_Shutdown, true);
}

void S_Shutdown(void)
{
    I_ShutdownSound();
    I_ShutdownMusic();
}

extern void StopAllSounds();

//
// Per level startup code.
// Kills playing sounds at start of level,
//  determines music if any, changes music.
//

void S_Start(void)
{
    int mnum;

    // kill all playing sounds at start of level
    //  (trust me - a good idea)
    StopAllSounds();

    // start new music for the level
    mus_paused = 0;

    if (gamemode == commercial)
    {
        mnum = mus_runnin + gamemap - 1;
    }
    else
    {
        int spmus[]=
        {
            // Song - Who? - Where?

            mus_e3m4,        // American     e4m1
            mus_e3m2,        // Romero       e4m2
            mus_e3m3,        // Shawn        e4m3
            mus_e1m5,        // American     e4m4
            mus_e2m7,        // Tim          e4m5
            mus_e2m4,        // Romero       e4m6
            mus_e2m6,        // J.Anderson   e4m7 CHIRON.WAD
            mus_e2m5,        // Shawn        e4m8
            mus_e1m9,        // Tim          e4m9
        };

        if (gameepisode < 4)
        {
            mnum = mus_e1m1 + (gameepisode-1)*9 + gamemap-1;
        }
        else
        {
            mnum = spmus[gamemap-1];
        }
    }

    S_ChangeMusic(mnum, true);
}

// clamp supplied integer to the range 0 <= x <= 255.

static int Clamp(int x)
{
    if (x < 0)
    {
        return 0;
    }
    else if (x > 255)
    {
        return 255;
    }
    return x;
}

//
// Stop and resume music, during game PAUSE.
//

void S_PauseSound(void)
{
    if (mus_playing && !mus_paused)
    {
        I_PauseSong();
        mus_paused = true;
    }
}

void S_ResumeSound(void)
{
    if (mus_playing && mus_paused)
    {
        I_ResumeSong();
        mus_paused = false;
    }
}

void S_SetMusicVolume(int volume)
{
    if (volume < 0 || volume > 127)
    {
        I_Error("Attempt to set music volume at %d",
                volume);
    }

    I_SetMusicVolume(volume);
}

struct SoundEngine {
	int global_volume;
	// ...
};

extern struct SoundEngine SOUND_ENGINE;

void S_SetSfxVolume(int volume)
{
    if (volume < 0 || volume > 127)
    {
        I_Error("Attempt to set sfx volume at %d", volume);
    }

    SOUND_ENGINE.global_volume = volume;
}

//
// Starts some music with the music id found in sounds.h.
//

void S_StartMusic(int m_id)
{
    S_ChangeMusic(m_id, false);
}

void S_ChangeMusic(int musicnum, int looping)
{
    musicinfo_t *music = NULL;
    char namebuf[9];
    void *handle;

    // The Doom IWAD file has two versions of the intro music: d_intro
    // and d_introa.  The latter is used for OPL playback.

    if (musicnum == mus_intro && (snd_musicdevice == SNDDEVICE_ADLIB
                               || snd_musicdevice == SNDDEVICE_SB)
        && W_CheckNumForName("D_INTROA") >= 0)
    {
        musicnum = mus_introa;
    }

    if (musicnum <= mus_None || musicnum >= NUMMUSIC)
    {
        I_Error("Bad music number %d", musicnum);
    }
    else
    {
        music = &S_music[musicnum];
    }

    if (mus_playing == music)
    {
        return;
    }

    // shutdown old music
    S_StopMusic();

    // get lumpnum if neccessary
    if (!music->lumpnum)
    {
        M_snprintf(namebuf, sizeof(namebuf), "d_%s", DEH_String(music->name));
        music->lumpnum = W_GetNumForName(namebuf);
    }

    music->data = W_CacheLumpNum(music->lumpnum, PU_STATIC);

    handle = I_RegisterSong(music->data, W_LumpLength(music->lumpnum));
    music->handle = handle;
    I_PlaySong(handle, looping);

    mus_playing = music;
}

boolean S_MusicPlaying(void)
{
    return I_MusicIsPlaying();
}

void S_StopMusic(void)
{
    if (mus_playing)
    {
        if (mus_paused)
        {
            I_ResumeSong();
        }

        I_StopSong();
        I_UnRegisterSong(mus_playing->handle);
        W_ReleaseLumpNum(mus_playing->lumpnum);
        mus_playing->data = NULL;
        mus_playing = NULL;
    }
}

