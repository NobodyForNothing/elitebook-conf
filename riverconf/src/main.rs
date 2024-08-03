// Requires the following software to be installed and in PATH:
// - riverctl, rivertile
// - brightnessctl - https://github.com/Hummer12007/brightnessctl
// - pamixer - https://github.com/cdemoulins/pamixer
// - foot - https://codeberg.org/dnkl/foot
// - dmenu-wl_run - https://github.com/nyyManni/dmenu-wayland
// - mako -  https://github.com/emersion/mako
// - sandbar - https://github.com/kolunmi/sandbar

use std::process::{Child, Command};

mod sandbar;

/// Used to invoke riverctl
const RIVERCTL_PATH: &'static str = "riverctl";

// ------- KEYS -------
const MOD_KEY: &'static str = "Super";
const TERMINAL_KEY: &'static str = "Q";
const MENU_KEY: &'static str = "W";
const CLOSE_APP_KEY: &'static str = "C";
const EXIT_KEY: &'static str = "N";
const FULLSCREEN_KEY: &'static str = "F";

// ------- APPS -------
const TERMINAL: &'static str = "foot";
const MENU: &'static str = "dmenu-wl_run -i";

// ------- THEMING -------
const BG_COLOR: &'static str = "0x000000";
const BORDER_COLOR_FOCUSED: &'static str = "0x93a1a1";
const BORDER_COLOR_UNFOCUSED: &'static str = "0x586e75";

// ------- FUNCTIONALITY -------
/// Key repeats per second.
const KEY_REPEAT_RATE: &'static str = "50";
const KEY_REPEAT_DELAY: &'static str = "250";
const KEYBOARD_LAYOUT: &'static str = "de";
const VOLUME_MODIFY_STEP: &'static str = "5";
const BRIGHTNESS_MODIFY_STEP: &'static str = "5";

/// Script like rust program to configure the river wm to my liking.
fn main() {

    let mut spawned: Vec<Child> = Vec::new();

    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("keyboard-layout")
        .arg(KEYBOARD_LAYOUT)
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("set-repeat")
        .arg(KEY_REPEAT_RATE)
        .arg(KEY_REPEAT_DELAY)
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("default-layout")
        .arg("rivertile")
        .spawn().unwrap());

    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("background-color")
        .arg(BG_COLOR)
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("border-color-focused")
        .arg(BORDER_COLOR_FOCUSED)
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("border-color-unfocused")
        .arg(BORDER_COLOR_UNFOCUSED)
        .spawn().unwrap());

    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("map")
        .arg("normal")
        .arg(MOD_KEY)
        .arg(TERMINAL_KEY)
        .arg("spawn")
        .arg(TERMINAL)
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("map")
        .arg("normal")
        .arg(MOD_KEY)
        .arg(MENU_KEY)
        .arg("spawn")
        .arg(MENU)
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("map")
        .arg("normal")
        .arg(MOD_KEY)
        .arg(CLOSE_APP_KEY)
        .arg("close")
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("map")
        .arg("normal")
        .arg(MOD_KEY)
        .arg(EXIT_KEY)
        .arg("exit")
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("map")
        .arg("normal")
        .arg(MOD_KEY)
        .arg(FULLSCREEN_KEY)
        .arg("toggle-fullscreen")
        .spawn().unwrap());
    for i in 1u8..9u8 {
        let tags = 1u8 << (i - 1);
        spawned.push(Command::new(RIVERCTL_PATH)
            .arg("map")
            .arg("normal")
            .arg(MOD_KEY)
            .arg(i.to_string())
            .arg("set-focused-tags")
            .arg(tags.to_string())
            .spawn().unwrap());
    }
    // Setup volume and brightness keys
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("map")
        .arg("normal")
        .arg("None")
        .arg("XF86AudioRaiseVolume")
        .arg("spawn")
        .arg(format!("'pamixer -i {}'", VOLUME_MODIFY_STEP))
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("map")
        .arg("normal")
        .arg("None")
        .arg("XF86AudioLowerVolume")
        .arg("spawn")
        .arg(format!("'pamixer -d {}'", VOLUME_MODIFY_STEP))
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("map")
        .arg("normal")
        .arg("None")
        .arg("XF86AudioMute")
        .arg("spawn")
        .arg("'pamixer --toggle-mute'")
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("map")
        .arg("normal")
        .arg("None")
        .arg("XF86MonBrightnessUp")
        .arg("spawn")
        .arg(format!("'brightnessctl set +{}%'", BRIGHTNESS_MODIFY_STEP))
        .spawn().unwrap());
    spawned.push(Command::new(RIVERCTL_PATH)
        .arg("map")
        .arg("normal")
        .arg("None")
        .arg("XF86MonBrightnessDown")
        .arg("spawn")
        .arg(format!("'brightnessctl set {}%-'", BRIGHTNESS_MODIFY_STEP))
        .spawn().unwrap());

    for mut p in spawned {
        let res = p.wait().unwrap();
        assert!(res.success());
    }

    // Layout
    Command::new("rivertile")
        .arg("-view-padding")
        .arg("1")
        .arg("-outer-padding")
        .arg("0")
        .spawn().unwrap();

    // Notifications
    Command::new("mako").spawn().unwrap();

    // Status bar
    sandbar::launch_sandbar();
}

