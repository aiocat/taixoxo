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

: check directory
if exist %APPDATA%\..\Local\osu!\Skins (
    : move skin to folder
    tar.exe -xf "Technosu Modded (v1.0).zip"
    robocopy "Technosu Modded (v1.0)" "%LocalAppData%\osu!\Skins\Technosu Modded (v1.0)" /E /MOVE

    : remove old folders
    del "Technosu Modded (v1.0).zip"
    echo Skin is loaded. Please open osu! and select "Technosu Modded (v1.0)".
) else (
    : give an error
    echo Can't find osu! skins path.
)

pause