use skyline::{hook, install_hooks};
use skyline::hooks::InlineCtx;
pub mod config;
pub use config::*;
use the_csk_collection_api;


static mut IS_PLAYING: bool = false;

// We are using a hook to a function for if the
// game is in the title screen's "Press any button state." 
#[hook(offset = 0x1bfaf50, inline)]
fn create_press_any_hook(ctx: &mut InlineCtx) {
	unsafe {
		// This function gets called multiple times, so
		// we use the IS_PLAYING boolean to check if we're already
		// Playing the bgm
		if !(IS_PLAYING) {
			// Load the TitleConfig file and get the hash we're
			// supposed to play
			let title_conf = TitleConfig::load_or_create();
			match title_conf {
				Ok(c) => {
					the_csk_collection_api::play_bgm(c.bgm_hash);
					IS_PLAYING = true;
				}
				Err(_e) => {
					// TO-DO find out how to raise error
				}
			}
		}
	}
}

// Hook for the "How to Play" state
// We need to mute the audio, so play silent audio
#[hook(offset = 0x1bfac90, inline)]
fn create_how_to_hook(ctx: &mut InlineCtx) {
	unsafe {
		IS_PLAYING = false;
		the_csk_collection_api::play_bgm(0x12129dab28); 
	}
}

// Hook for leaving the title screen
// Change the IS_PLAYING boolean
#[hook(offset = 0x1bfb290, inline)]
fn create_exit_hook(ctx: &mut InlineCtx) {
	// so that when we go back to the title screen
	// it will start playing again.
	unsafe {
		IS_PLAYING = false;
	}
}

#[skyline::main(name = "PlayBGMonTitleScreen")]
pub fn main() {
    install_hooks!(create_press_any_hook, 
				   create_exit_hook, 
				   create_how_to_hook,);
}