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

use bitflags::bitflags;
use ffi::*;

bitflags! {
    pub struct Flags: u32 {
        const NO_FLAGS = XKB_COMPOSE_COMPILE_NO_FLAGS;
    }
}

pub const NO_FLAGS: Flags = Flags::NO_FLAGS;

impl Default for Flags {
    fn default() -> Self {
        NO_FLAGS
    }
}
