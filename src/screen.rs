// Copyright (C) 2022 aiocat
//
// This file is part of taixoxo.
//
// taixoxo is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// taixoxo is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with taixoxo.  If not, see <http://www.gnu.org/licenses/>.

use winapi::shared::windef::HDC;
use winapi::um::wingdi::{GetBValue, GetGValue, GetPixel, GetRValue};
use winapi::um::winuser::GetDC;

use std::ptr;

pub type Screen = HDC; // alias for HDC

// get screen function
pub fn get_screen() -> Screen {
    unsafe { GetDC(ptr::null_mut()) }
}

// free screen function for memory safety (unused for now)
/*
    pub fn free_screen(screen: Screen) -> bool {
        (unsafe { ReleaseDC(ptr::null_mut(), screen) }) as u8 == 1
    }
*/

// get pixel color from x and y position
pub fn get_pixel(screen: Screen, x: i32, y: i32) -> (u8, u8, u8) {
    let color = unsafe { GetPixel(screen, x, y) };
    (GetRValue(color), GetGValue(color), GetBValue(color))
}
