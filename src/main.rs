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

fn main() {
    let screen_handle = screen::get_screen();
    let (r, g, b) = screen::get_pixel(screen_handle, 27, 155);
    println!("RGB({}, {}, {})", r, g, b);
    screen::free_screen(screen_handle);
}
