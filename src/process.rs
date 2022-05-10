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

use winapi::um::winuser::{GetWindowThreadProcessId, GetForegroundWindow, GetWindowRect};
use winapi::shared::windef::{HWND, RECT};

use sysinfo::{PidExt, ProcessExt, System, SystemExt};

pub type Rectange = RECT; // alias for RECT
pub type Window = HWND; // alias for HWND

// create new rect
fn new_rect() -> RECT {
    Rectange{
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    }
}

// get active window wrapper
pub fn get_active_window() -> Option<Window> {
    let window = unsafe { GetForegroundWindow() };

    if window.is_null() {
        return None;
    }

    Some(window)
}

// check if window has same pid
pub fn is_same_pid(window: Window, excepted: u32) -> bool {
    let mut process_id = 0;
    unsafe { GetWindowThreadProcessId(window, &mut process_id) };

    if process_id == 0 {
        panic!("Can't find thread pid.");
    }

    process_id == excepted
}

// get window position
pub fn get_window_position(window: Window) -> Rectange {
    let mut rectange = new_rect();

    if unsafe { GetWindowRect(window, &mut rectange) } == 0 {
        panic!("Can't get window positions.");
    }

    rectange
}

// get osu pid
pub fn get_osu_pid() -> u32 {
    let mut system = System::new_all();
    system.refresh_all();

    for (pid, process) in system.processes() {
        if process.name() == "osu!.exe" {
            return pid.as_u32()
        }
    }

    0
}