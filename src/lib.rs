use std::sync::atomic::{self, AtomicBool};

use skyline::hooks::InlineCtx;
use skyline::{hook, install_hooks};
use hash40::Hash40;

use once_cell::sync::Lazy;

pub mod config;
pub use config::*;
pub mod offsets;
pub use offsets::*;

static IS_PLAYING: AtomicBool = AtomicBool::new(false);
static TITLE_CONFIG: Lazy<TitleConfig> = Lazy::new(|| {
    let title_conf = TitleConfig::load_or_create();
        match title_conf {
            Ok(c) => {
                c
            }
            Err(_e) => {
                TitleConfig::new()
            }
        }
});

// We are using a hook to a function for if the
// game is in the title screen's "Press any button state."
#[hook(offset = offsets::PRESS_ANY_BUTTON_OFFSET, inline)]
fn create_press_any_hook(ctx: &mut InlineCtx) {
    // This function gets called multiple times, so
    // we use the IS_PLAYING boolean to check if we're already
    // Playing the bgm
    if !(IS_PLAYING.load(atomic::Ordering::Relaxed)) {
        // Play the bgm in the config file
        the_csk_collection_api::play_bgm(TITLE_CONFIG.ui_bgm_id.0);
        IS_PLAYING.store(true, atomic::Ordering::Relaxed);
    }
}

// Hook for the "How to Play" state
// We need to mute the audio, so play silent audio
#[hook(offset = offsets::HOW_TO_PLAY_OFFSET, inline)]
fn create_how_to_hook(ctx: &mut InlineCtx) {
    IS_PLAYING.store(false, atomic::Ordering::Relaxed);

    let bgm_hash = Hash40::new("ui_bgm_crs2_49_commonsilent_lp");
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

// Hook into the title screen initialization function
// and modify the register for the title screen time out value
#[hook(offset = offsets::TITLE_SCREEN_INIT_OFFSET, inline)]
fn create_title_init_hook(ctx: &mut InlineCtx) {
    if (TITLE_CONFIG.disable_timeout.load(atomic::Ordering::Relaxed))
    {
        unsafe {
            // This is the max value that can be used as the title screen
            // time out value. This is essentially ~13 months worth of time,
            // so functionally infinite
            *ctx.registers[10].w.as_mut() = i32::MAX as u32;
        }
    }
}

#[skyline::main(name = "PlayBGMonTitleScreen")]
pub fn main() {
    install_hooks!(create_press_any_hook, create_exit_hook, create_how_to_hook, create_title_init_hook);
}
