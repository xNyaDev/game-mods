#!/usr/bin/env bash

cross +nightly run --target=i686-pc-windows-gnu -p xnya_modloader_gen -- dinput8.dll --out target/xnya_modloader_dinput8_i686 --wine

cd target/xnya_modloader_dinput8_i686
cross +nightly build --target=i686-pc-windows-gnu --release
cd ../..

cross +nightly build --target=x86_64-pc-windows-gnu --release --package xnya_console_enabler

cross +nightly build --target=i686-pc-windows-gnu --release --package xnya_rallytrophy_cryptutil

mkdir dist
mkdir "dist/Rally Trophy"

cp target/x86_64-pc-windows-gnu/release/xnya_console_enabler.exe dist

cp target/xnya_modloader_dinput8_i686/target/i686-pc-windows-gnu/release/xnya_modloader.dll "dist/Rally Trophy/dinput8.dll"
cp target/xnya_modloader_dinput8_i686/target/xnya_modloader.toml "dist/Rally Trophy"
cp target/i686-pc-windows-gnu/release/xnya_rallytrophy_cryptutil.dll "dist/Rally Trophy"
cp target/xnya_rallytrophy_cryptutil.toml "dist/Rally Trophy"

echo Build complete