: Copyright (C) 2022 aiocat
: 
: This file is part of taixoxo.
: 
: taixoxo is free software: you can redistribute it and/or modify
: it under the terms of the GNU General Public License as published by
: the Free Software Foundation, either version 3 of the License, or
: (at your option) any later version.
: 
: taixoxo is distributed in the hope that it will be useful,
: but WITHOUT ANY WARRANTY; without even the implied warranty of
: MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
: GNU General Public License for more details.
: 
: You should have received a copy of the GNU General Public License
: along with taixoxo.  If not, see <http://www.gnu.org/licenses/>.

: close echo
@echo off

: format code before compile
cargo fmt

: remove old files if folder exists
if exist .\build (
    rmdir /Q /S .\build 
) else (
    mkdir .\build
)

: compile for windows-gnu
cls

: set override and compile
rustup override set stable-x86_64-pc-windows-gnu
cargo build --release

: create gnu build folder
mkdir .\build\taixoxo

: move file to folder
move .\target\release\taixoxo.exe .\build\taixoxo\taixoxo.exe

: copy license, readme, skin and skin setup script
echo f | xcopy /f /y .\README.md .\build\taixoxo\README.md
echo f | xcopy /f /y .\LICENSE .\build\taixoxo\LICENSE
echo f | xcopy /f /y .\assets\setup-skin.bat .\build\taixoxo\setup-skin.bat
echo f | xcopy /f /y ".\assets\Technosu Modded (v1.0).zip" ".\build\taixoxo\Technosu Modded (v1.0).zip"

: into build folder
cd .\build

: generate sha256 hash
certutil -hashfile ".\taixoxo\taixoxo.exe" SHA256 >> .\taixoxo\checksum.txt

: create zip
tar.exe -a -cf taixoxo.zip taixoxo

: delete folder if argument given
if "%1" == "clean" (
    rmdir /Q /S .\taixoxo
)

: move to source folder
cd ..

: clear override
rustup override unset