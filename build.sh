#!/usr/bin/env bash

cross +nightly run --target=i686-pc-windows-gnu -p xnya_modloader_gen -- dinput8.dll --out target/xnya_modloader_dinput8_i686 --wine
cross +nightly run --target=i686-pc-windows-gnu -p xnya_modloader_gen -- winmm.dll --out target/xnya_modloader_winmm_i686 --wine

cd target/xnya_modloader_dinput8_i686
cross +nightly build --target=i686-pc-windows-gnu --release
cd ../..
cd target/xnya_modloader_winmm_i686
cross +nightly build --target=i686-pc-windows-gnu --release
cd ../..

cross +nightly build --target=x86_64-pc-windows-gnu --release --package xnya_console_enabler

cross +nightly build --target=i686-pc-windows-gnu --release --package xnya_rallytrophy_cryptutil
cross +nightly build --target=i686-pc-windows-gnu --release --package xnya_retrodemo_cryptutil

mkdir dist
mkdir "dist/Rally Trophy"
mkdir "dist/Bugbear Retro Demo 2002"

cp target/x86_64-pc-windows-gnu/release/xnya_console_enabler.exe dist

cp target/xnya_modloader_dinput8_i686/target/i686-pc-windows-gnu/release/xnya_modloader.dll "dist/Rally Trophy/dinput8.dll"
cp target/xnya_modloader_dinput8_i686/target/xnya_modloader.toml "dist/Rally Trophy"
cp target/i686-pc-windows-gnu/release/xnya_rallytrophy_cryptutil.dll "dist/Rally Trophy"
cp target/xnya_rallytrophy_cryptutil.toml "dist/Rally Trophy"

cp target/xnya_modloader_winmm_i686/target/i686-pc-windows-gnu/release/xnya_modloader.dll "dist/Bugbear Retro Demo 2002/winmm.dll"
cp target/xnya_modloader_winmm_i686/target/xnya_modloader.toml "dist/Bugbear Retro Demo 2002"
cp target/i686-pc-windows-gnu/release/xnya_retrodemo_cryptutil.dll "dist/Bugbear Retro Demo 2002"
cp target/xnya_retrodemo_cryptutil.toml "dist/Bugbear Retro Demo 2002"

echo Build complete