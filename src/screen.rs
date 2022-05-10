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

use winapi::um::wingdi::{GetPixel, GetRValue, GetGValue, GetBValue};
use winapi::um::winuser::{ReleaseDC, GetDC};
use winapi::shared::windef::HDC;

use std::ptr;

pub type SCREEN = HDC; // screen alias for HDC

// get screen function
pub fn get_screen() -> SCREEN {
    unsafe { GetDC(ptr::null_mut()) }
}

// free screen function for memory safety
pub fn free_screen(screen: SCREEN) -> bool {
    (unsafe { ReleaseDC(ptr::null_mut(), screen) }) as u8 == 1
}

// get pixel color from x and y position
pub fn get_pixel(screen: SCREEN, x: i32, y: i32) -> (u8, u8, u8) {
    let color = unsafe { GetPixel(screen, x, y) };
    (GetRValue(color), GetGValue(color), GetBValue(color))
}