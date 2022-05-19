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
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

use winput::message_loop;
use winput::{Action, Vk};

#[derive(std::cmp::PartialEq, Debug)]
enum TaixoxoStatus {
    Enabled,
    Disabled,
    Calculating,
    Closing,
}

#[derive(Debug)]
struct Taixoxo {
    window: process::Window,
    screen: screen::Screen,
    position_x: i32,
    position_y: i32,
}

fn app_panic(msg: &str) {
    println!("[ERROR] {}", msg);
    sleep(Duration::from_secs(3));
    exit(1);
}

fn app_info(msg: &str) {
    println!("[INFO] {}", msg);
}

impl Taixoxo {
    // create new taixoxo instance
    fn new() -> Taixoxo {
        Taixoxo {
            window: null_mut(),
            screen: null_mut(),
            position_x: 0,
            position_y: 0,
        }
    }

    // get osu! window
    fn get_window(&mut self) {
        let screen_handle = screen::get_screen();
        let osu_pid = process::get_osu_pid();
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

        self.window = window_handle;
        self.screen = screen_handle;
    }

    // calculate bot position using window position
    fn calculate_position(&mut self) {
        let position = process::get_window_position(self.window);
        let size = (
            position.bottom - position.top,
            position.right - position.left,
        );
        // println!("{:?}", size);
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
                pos_x += 242;
                pos_y += 330;
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
                app_panic("You are not in supported window size. Switch osu! to window mode and set your window size to one of the following options:
      - (800x600)
      - (1024x768) [recommended]
      - (1152x864)
      - (1920x1080 borderless) [most recommended]
      - (1366x768) [recommended]")
            }
        }

        self.position_x = pos_x;
        self.position_y = pos_y;
    }

    pub fn start(&mut self) {
        // constants for red, blue and yellow taiko
        let (blue_r, blue_g, blue_b) = (60..70, 130..140, 140..180);
        let (red_r, red_g, red_b) = (200..255, 60..70, 40..50);
        let (yellow_r, yellow_g, yellow_b) = (200..255, 150..190, 0..30);

        // title
        println!("Taixoxo v1.2.0 =>");
        println!("- Source Code: github.com/aiocat/taixoxo");
        println!("- License: GNU General Public License v3\n");

        // info message
        println!("[INFO] Please focus on your osu! window in 5 seconds. (and don't move your osu! window)");
        sleep(Duration::from_secs(5));

        // calculate
        self.get_window();
        self.calculate_position();

        // get size and position
        app_info("Bot initialized! Please don't move your osu! window.");

        // run another thread to wait for keys
        // manage bot status
        let status = Arc::new(Mutex::new(TaixoxoStatus::Enabled));
        let thread_status = Arc::clone(&status);
        spawn(move || {
            Taixoxo::handle_keys(thread_status);
        });

        // main part of the bot
        let mut need_to_click = true;
        loop {
            // check bot status
            {
                let mut temp_status = status.lock().unwrap();
                if *temp_status == TaixoxoStatus::Disabled {
                    continue;
                } else if *temp_status == TaixoxoStatus::Closing {
                    break;
                } else if *temp_status == TaixoxoStatus::Calculating {
                    self.get_window();
                    self.calculate_position();
                    *temp_status = TaixoxoStatus::Enabled;
                }
            }

            let (r, g, b) = screen::get_pixel(self.screen, self.position_x, self.position_y);

            // check pixel
            if red_r.contains(&r) && red_g.contains(&g) && red_b.contains(&b) && need_to_click {
                input::press_for_red();
                need_to_click = false;
            } else if blue_r.contains(&r)
                && blue_g.contains(&g)
                && blue_b.contains(&b)
                && need_to_click
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

        screen::free_screen(self.screen);
    }

    fn handle_keys(data: Arc<Mutex<TaixoxoStatus>>) {
        let receiver = message_loop::start().unwrap();

        loop {
            if let message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } = receiver.next_event()
            {
                if vk == Vk::Alt {
                    let mut thread_bot_status = data.lock().unwrap();
                    if *thread_bot_status == TaixoxoStatus::Enabled {
                        *thread_bot_status = TaixoxoStatus::Disabled;
                        app_info("Bot is currently disabled. Press \"Alt\" to enable bot.");
                    } else {
                        *thread_bot_status = TaixoxoStatus::Enabled;
                        app_info("Bot is currently enabled. Press \"Alt\" to disable bot.");
                    }
                } else if vk == Vk::Home {
                    let mut thread_bot_status = data.lock().unwrap();
                    if *thread_bot_status != TaixoxoStatus::Enabled {
                        app_info("You need to enable bot to calculate osu! position again.");
                    } else {
                        *thread_bot_status = TaixoxoStatus::Calculating;
                        app_info("Re-calculated the position!");
                    }
                } else if vk == Vk::End {
                    let mut thread_bot_status = data.lock().unwrap();
                    app_info("Closing the bot...");
                    *thread_bot_status = TaixoxoStatus::Closing;
                }
            }
        }
    }
}

fn main() {
    let mut bot = Taixoxo::new();
    bot.start();
}
