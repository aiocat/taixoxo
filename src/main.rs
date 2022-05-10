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
mod screen;
mod process;

fn main() {
    let screen_handle = screen::get_screen();
    let osu_pid = process::get_osu_pid();

    let (blue_r, blue_g, blue_b) = (60..70, 130..140, 140..180);
    let (red_r, red_g, red_b) = (200..255, 60..70, 40..50);

    loop {
        let window_handle = process::get_active_window();
        if window_handle.is_none() {
            continue
        }

        let window_handle = window_handle.unwrap(); // shadow variable

        if !process::is_same_pid(window_handle, osu_pid) {
            continue
        }

        let position = process::get_window_position(window_handle);

        if (position.bottom - position.top, position.right - position.left) != (629, 806) {
            println!("Set window size properly! (800x600)");
            break;
        }

        let (r, g, b) = screen::get_pixel(screen_handle, position.left + 170, position.top + 245);

        // check pixel
        if blue_r.contains(&r) && blue_g.contains(&g) && blue_b.contains(&b) {
            println!("Found Blue!");
        } else if red_r.contains(&r) && red_g.contains(&g) && red_b.contains(&b) {
            println!("Found Red!");
        }
    }

    screen::free_screen(screen_handle);
}
