use common::{Z_Free, fixed::fixed};

use crate::{defs::MAX_PLAYERS, stat::*};

#[repr(C)]
pub struct Thinker {
    prev: *mut Thinker,
    next: *mut Thinker,
    func: *const std::ffi::c_void
}

impl Thinker {
    fn invoke_acp1(&mut self) {
        unsafe {
            invoke_acp1(self.func, self);
        }
    }

    fn iterate<'a>(&'a self) -> ThinkerIter<'a> {
        ThinkerIter { current: self }
    }
}

pub struct ThinkerIter<'a> {
    current: &'a Thinker
}

impl<'a> Iterator for ThinkerIter<'a> {
    type Item = &'static mut Thinker;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.current.next != &raw mut thinkercap {
                let next = self.current.next;
                self.current = &*next;
				
                Some(std::mem::transmute(next))
            } else {
                None
            }
        }
    }
}

unsafe extern "C" {
    pub static mut thinkercap: Thinker;

    fn invoke_acp1(func: *const std::ffi::c_void, thinker: *mut Thinker);

    fn P_UpdateSpecials();
    fn P_RespawnSpecials();
}

#[allow(dangling_pointers_from_temporaries)]
pub const REM_FUNC: *const std::ffi::c_void = std::ptr::without_provenance_mut::<std::ffi::c_void>(usize::MAX);

#[unsafe(no_mangle)]
pub extern "C" fn P_InitThinkers() {
    unsafe {
        thinkercap.prev = &raw mut thinkercap;
        thinkercap.next = &raw mut thinkercap;
    }
}

//
// Adds a new thinker at the end of the list.
//
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P_AddThinker(thinker: *mut Thinker) {
    unsafe
    {
        (*thinkercap.prev).next = thinker;
        (*thinker).next = &raw mut thinkercap;
        (*thinker).prev = thinkercap.prev;
        thinkercap.prev = thinker;
    }
}

//
// P_RemoveThinker
// Deallocation is lazy -- it will not actually be freed
// until its thinking turn comes up.
//
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P_RemoveThinker(thinker: *mut Thinker)
{
    // FIXME: NOP.
    unsafe {
        (*thinker).func = REM_FUNC;
    }
}

//
// P_AllocateThinker
// Allocates memory and adds a new thinker at the end of the list.
//
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P_AllocateThinker (_: *mut Thinker)
{
}

#[allow(static_mut_refs)]
unsafe fn run_thinkers()
{
    unsafe {
        let mut thinkers_to_remove: Vec<*mut Thinker> = Vec::new();

        for thinker in thinkercap.iterate()
        {
            if thinker.func == REM_FUNC
            {
                thinkers_to_remove.push(thinker);
            }
            else if !thinker.func.is_null()
            {
                thinker.invoke_acp1();
            }
        }

        // time to remove it
        for thinker in thinkers_to_remove
        {
            (*(*thinker).next).prev = (*thinker).prev;
            (*(*thinker).prev).next = (*thinker).next;

            Z_Free(thinker as *mut std::ffi::c_void);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn P_Ticker()
{
    unsafe {
        // run the tic
        if paused {
            return;
        }
            
        // pause if in menu and at least one tic has been run
        if !netgame
            && menuactive
            && !demoplayback
            && players[consoleplayer as usize].viewz != fixed(1)
        {
            return;
        }
        
            
        for i in 0..MAX_PLAYERS {
            if playeringame[i] != 0 {
                players[i].think();
            }
        }
                
        run_thinkers ();
        P_UpdateSpecials ();
        P_RespawnSpecials ();

        // for par times
        leveltime += 1;
    }	
}