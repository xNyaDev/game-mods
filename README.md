# xNyaDev's game mods

This repo contains all of my native game mods written in Rust. 

Game list:
 - Rally Trophy - Patch 1.01 EN/DE

## Building instructions

Rust nightly MSVC (i686 and x86_64) is required to build. Cross-compilation or using the GNU compiler is not
supported and will cause issues - `xnya_modloader_gen` requires a Windows environment to read exports of proxied DLLs.

Run `build.bat` to build all the mods. Ready packages will be placed in /dist, sorted by the game name.

# Project list

## xnya_console_enabler

Tries to turn a GUI program into a console app by changing the PE header from Windows GUI into Windows Console.

## xnya_modloader_gen and xnya_modloader_template

xnya_modloader_gen creates a proxy DLL project by reading exports of a specified DLL and adding them to
xnya_modloader_template where required. The resulting project is not compiled and can be customized further if desired.

## xnya_rallytrophy_cryptutil

Game: Rally Trophy

This mod can dump the key from Rally Trophy and make the game run completely decrypted if so desired. Designed for use
with [bfstool](https://github.com/xNyaDev/bfstool), exports the key in a compatible format.

## xnya_util

Contains utilities like config formats or functions to read/write TOML files