#![no_std]

use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
pub extern "C" fn init() {
    // 我的代码
}

#[no_mangle]
pub extern "C" fn handle() {
    // 我的代码
}

#[no_mangle]
pub extern "C" fn state() {
    // 我的代码
}