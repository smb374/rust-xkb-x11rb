//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

pub mod mods {
    use ffi::*;

    pub const SHIFT: &[u8] = XKB_MOD_NAME_SHIFT;
    pub const CAPS: &[u8] = XKB_MOD_NAME_CAPS;
    pub const CTRL: &[u8] = XKB_MOD_NAME_CTRL;
    pub const ALT: &[u8] = XKB_MOD_NAME_ALT;
    pub const NUM: &[u8] = XKB_MOD_NAME_NUM;
    pub const LOGO: &[u8] = XKB_MOD_NAME_LOGO;
}

pub mod leds {
    use ffi::*;

    pub const CAPS: &[u8] = XKB_LED_NAME_CAPS;
    pub const NUM: &[u8] = XKB_LED_NAME_NUM;
    pub const SCROLL: &[u8] = XKB_LED_NAME_SCROLL;
}
