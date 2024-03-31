use std::sync::atomic::{self, AtomicBool};

use skyline::hooks::InlineCtx;
use skyline::{hook, install_hooks};

pub mod config;
pub use config::*;
pub mod offsets;
pub use offsets::*;

static IS_PLAYING: AtomicBool = AtomicBool::new(false);

// We are using a hook to a function for if the
// game is in the title screen's "Press any button state."
#[hook(offset = offsets::PRESS_ANY_BUTTON_OFFSET, inline)]
fn create_press_any_hook(ctx: &mut InlineCtx) {
    // This function gets called multiple times, so
    // we use the IS_PLAYING boolean to check if we're already
    // Playing the bgm
    if !(IS_PLAYING.load(atomic::Ordering::Relaxed)) {
        // Load the TitleConfig file and get the hash we're
        // supposed to play
        let title_conf = TitleConfig::load_or_create();
        match title_conf {
            Ok(c) => {
                the_csk_collection_api::play_bgm(c.ui_bgm_id.0);
                IS_PLAYING.store(true, atomic::Ordering::Relaxed);
            }
            Err(_e) => {
                // TO-DO find out how to raise error
            }
        }
    }
}

// Hook for the "How to Play" state
// We need to mute the audio, so play silent audio
#[hook(offset = offsets::HOW_TO_PLAY_OFFSET, inline)]
fn create_how_to_hook(ctx: &mut InlineCtx) {
    IS_PLAYING.store(false, atomic::Ordering::Relaxed);
    let bgm_hash = Result::expect(
        hash40::Hash40::from_label("m21b_gaw_gamer_mom"),
        "Could not convert label to Hash40!",
    );
    the_csk_collection_api::play_bgm(bgm_hash.0);
}

// Hook for leaving the title screen
// Change the IS_PLAYING boolean
#[hook(offset = offsets::TITLE_SCREEN_EXIT_OFFSET, inline)]
fn create_exit_hook(ctx: &mut InlineCtx) {
    // so that when we go back to the title screen
    // it will start playing again.
    IS_PLAYING.store(false, atomic::Ordering::Relaxed);
}

#[skyline::main(name = "PlayBGMonTitleScreen")]
pub fn main() {
    install_hooks!(create_press_any_hook, create_exit_hook, create_how_to_hook,);
}
