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
mod input;
mod process;
mod screen;

use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let screen_handle = screen::get_screen();
    let osu_pid = process::get_osu_pid();

    // constants for red, blue and yellow taiko
    let (blue_r, blue_g, blue_b) = (60..70, 130..140, 140..180);
    let (red_r, red_g, red_b) = (200..255, 60..70, 40..50);
    let (yellow_r, yellow_g, yellow_b) = (210..255, 160..180, 0..20);

    // title
    println!("Taixoxo v1.1.0 =>");
    println!("- Source Code: github.com/aiocat/taixoxo");
    println!("- License: GNU General Public License v3\n");

    // info message
    println!(
        "[INFO] Please focus on your osu! window in 5 seconds. (and don't move your osu! window)"
    );
    sleep(Duration::from_secs(5));

    // get active window
    let window_handle = process::get_active_window();
    if window_handle.is_none() {
        app_panic("Can't find focused window!")
    }

    // check if window is osu
    let window_handle = window_handle.unwrap(); // shadow variable
    if !process::is_same_pid(window_handle, osu_pid) {
        app_panic("Thats not osu! >:(")
    }

    // get size and position
    let position = process::get_window_position(window_handle);
    let size = (
        position.bottom - position.top,
        position.right - position.left,
    );
    println!("{:?}", size);

    // x and y position for pixel
    let mut pos_x = position.left;
    let mut pos_y = position.top;

    // edit position for size
    match size {
        (629, 806) | (600, 800) | (601, 800) => {
            pos_x += 190;
            pos_y += 265;
        }
        (797, 1030) | (768, 1024) | (769, 1024) => {
            pos_x += 240;
            pos_y += 330;
        }
        (797, 1372) | (768, 1366) | (769, 1366) => {
            pos_x += 252;
            pos_y += 325;
        }
        (893, 1158) | (864, 1152) | (865, 1152) => {
            pos_x += 260;
            pos_y += 360;
        }
        (1081, 1920) | (1080, 1920) => {
            pos_x = 330;
            pos_y = 450;
        }
        _ => {
            pos_x = 330;
            pos_y = 450;

            println!("You are not in supported mode. Switch osu! to window mode and set your window size to one of the following options:\n  - (800x600)\n  - (1024x768) [recommended]\n  - (1152x864)\n  - (1920x1080 borderless) [recommended]\n  - (1366x768) [experimental]")
        }
    }

    println!("[INFO] Bot initialized! Please don't move your osu! window.");

    // main part of the bot
    let mut need_to_click = true;
    loop {
        let (r, g, b) = screen::get_pixel(screen_handle, pos_x, pos_y);

        // check pixel
        if red_r.contains(&r) && red_g.contains(&g) && red_b.contains(&b) && need_to_click {
            input::press_for_red();
            need_to_click = false;
        } else if blue_r.contains(&r) && blue_g.contains(&g) && blue_b.contains(&b) && need_to_click
        {
            input::press_for_blue();
            need_to_click = false;
        } else if yellow_r.contains(&r)
            && yellow_g.contains(&g)
            && yellow_b.contains(&b)
            && need_to_click
        {
            input::press_for_yellow();
            need_to_click = false;
        } else {
            need_to_click = true;
        }
    }
}

fn app_panic(msg: &str) {
    println!("[ERROR] {}", msg);
    sleep(Duration::from_secs(3));
    exit(1);
}
