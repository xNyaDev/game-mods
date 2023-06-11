# xNyaDev's game mods

This repo contains all of my native game mods written in Rust. 

Game list:
 - Rally Trophy

## Building instructions

Rust nightly MSVC (both i686 and x86_64) is required to build. Cross-compilation or using the GNU compiler is not
tested and might cause issues.

Run `build.bat` to build all the mods. It will download [ThirteenAG's Ultimate ASI Loader](https://github.com/ThirteenAG/Ultimate-ASI-Loader)
and configure it correctly for all the games. Ready packages will be placed in /dist, sorted by the game name.

# Mod list

## xnya_modloader

Currently, only purpose of this mod is to load other mods. It will load all DLLs matching `xnya_*.dll` and should be
named `xnya_modloader.asi` to be loaded by Ultimate ASI Loader into the game.

## xnya_rallytrophy_cryptutil

Compatibility: Rally Trophy 1.01 EN/DE 

This mod can dump the key from Rally Trophy and make the game run completely decrypted if so desired. Designed for use
with [bfstool](https://github.com/xNyaDev/bfstool)<sup>1</sup>, exports the key in a compatible format.

1. As of right now, bfstool has no decryption support, but it is planned for the near future.