# Play BGM on Title Screen
A rust plugin for Smash Ultimate to play music on the title screen

<p align="center">
    <a href="https://www.paypal.me/Struggleton/">
        <img src="https://cdn.rawgit.com/twolfson/paypal-github-button/1.0.0/dist/button.svg" width="155" alt="">
    </a>
    <a href="https://www.patreon.com/Struggleton">
        <img src="https://c5.patreon.com/external/logo/become_a_patron_button@2x.png" width="150" alt="">
    </a>
 <a href="https://ko-fi.com/Struggleton">
        <img src="https://uploads-ssl.webflow.com/5c14e387dab576fe667689cf/61e11d430afb112ea33c3aa5_Button-1-p-500.png" width="235" alt="">
    </a>
</p>

<p align="center">
    <a href="https://somsubhra.github.io/github-release-stats/?username=Struggleton&repository=TitleScreenBGM">
        <img src="https://img.shields.io/github/downloads/Struggleton/TitleScreenBGM/total" alt="">
    </a>
</p>


## Prerequisites
- [ARCropolis (Latest)](https://github.com/Raytwo/ARCropolis/releases/latest "ARCropolis (Latest)")
- [CSK Collection plugin](https://gamebanana.com/mods/499008 "The CSK Collection API")

## Setup
Download the latest version of this plugin and the CSK Collection plugin and place them under the`sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins`directory. 

## Usage
In the `sd:/ultimate` folder, create a file called `config_title.toml` (or the plugin will create one for you if it doesn't exist when the game starts.) Inside the toml file which can be opened in any text editor, write a line with the ui_bgm_id of the bgm you want to play.

#### Example (config_title.toml) - Plays "Classic: Final Results"
`bgm_hash = 0x209cc21ee3` 

## Credits
- @jugeeya for advice on inline function hooks in Rust
- @Coolsonickirby for creating the CSK Collection plugin and functions for developers