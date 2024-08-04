#![no_std]
#![feature(type_alias_impl_trait, const_async_blocks)]
#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::undocumented_unsafe_blocks,
    rust_2018_idioms
)]

use core::cmp::max;


use asr::{
    print_limited,
    emulator::gcn::{self, Emulator},
    future::{next_tick, retry},
    watcher::Watcher, timer::{self, TimerState}, time::Duration, time_util::frame_count, print_message, Address32, Address,
};
use asr::time_util;

use bitflags::bitflags;

asr::panic_handler!();
asr::async_main!(nightly);

const TARGET_GAME_ID : u32 = 0x47455A45;
async fn main() {

    loop {
        // Hook to the target process
        let mut emulator = retry(|| gcn::Emulator::attach()).await;
        let mut watchers = Watchers::default();
        let offsets = Offsets::new();

        loop {
            if !emulator.is_open() {

                break;
            }
            if emulator.update() {
                
                // Splitting logic. Adapted from OG LiveSplit:
                // Order of execution
                // 1. update() will always be run first. There are no conditions on the execution of this action.
                // 2. If the timer is currently either running or paused, then the isLoading, gameTime, and reset actions will be run.
                // 3. If reset does not return true, then the split action will be run.
                // 4. If the timer is currently not running (and not paused), then the start action will be run.
                update_loop(&emulator, &offsets, &mut watchers);

                let timer_state = timer::state();
                if timer_state == TimerState::Running {
                    

                    /*if let Some(game_time) = game_time(&watchers, &mut igt_info) {
                        
                        timer::set_game_time(game_time)
                    }*/
                    if split(&watchers) {
                        timer::split()
                    }
                }

                if timer::state() == TimerState::NotRunning {

                   
                    
                    if start(&watchers) {
                    timer::start();
                    timer::pause_game_time();
                    }
                    
                }
            }
            next_tick().await;
        }
    }
}



#[derive(Default)]
struct Watchers {
    game_id: Watcher<u32>,
    results_active: Watcher<u8>,
}
struct Offsets {
    game_id: u32,
    results_active: u32,

}

impl Offsets {
    fn new() -> Self {
        Self {
            results_active: 0x80736AF0,
            game_id: 0x80000000
            
        }
    }
}


fn update_loop(game: &Emulator, offsets: &Offsets, watchers: &mut Watchers) {

    let results_active = game.read::<u8>(offsets.results_active).unwrap_or_default();   
    let game_id = game.read::<u32>(offsets.game_id).unwrap_or_default(); 
    watchers.results_active.update_infallible(results_active);
    watchers.game_id.update_infallible(game_id);
    
    

}
    

fn start(watchers: &Watchers) -> bool {
    return false;
}

fn split(watchers: &Watchers) -> bool {

    if watchers.game_id.pair.expect("WHOOPS (GAMEID)").current != TARGET_GAME_ID {
        return false;
    }
    if watchers.results_active.pair.expect("WHOOPS (SPLIT)").changed_from(&0) {
        return true;
    }
    return false;
}


fn is_loading(watchers: &Watchers) -> Option<bool> {                                                       
    Some(false)
}
