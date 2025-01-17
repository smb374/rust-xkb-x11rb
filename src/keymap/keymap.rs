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

use crate::keymap::{Key, Layouts, Leds, Mods};
use crate::state::State;
use crate::Keycode;
use ffi::*;

#[derive(Debug)]
pub struct Keymap(*mut xkb_keymap);

impl Keymap {
    pub unsafe fn from_ptr(ptr: *mut xkb_keymap) -> Self {
        Keymap(ptr)
    }

    pub unsafe fn as_ptr(&self) -> *mut xkb_keymap {
        self.0
    }

    pub fn mods(&self) -> Mods {
        Mods(self)
    }

    pub fn layouts(&self) -> Layouts {
        Layouts(self)
    }

    pub fn leds(&self) -> Leds {
        Leds(self)
    }

    pub fn key(&self, key: Keycode) -> Key {
        Key(self, key)
    }

    pub fn state(&self) -> State {
        unsafe { State::from_ptr(xkb_state_new(self.0)) }
    }
}

impl Clone for Keymap {
    fn clone(&self) -> Self {
        unsafe { Keymap(xkb_keymap_ref(self.0)) }
    }
}

impl Drop for Keymap {
    fn drop(&mut self) {
        unsafe {
            xkb_keymap_unref(self.0);
        }
    }
}
