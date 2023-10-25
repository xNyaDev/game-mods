@echo off

cargo +nightly run --target=i686-pc-windows-msvc -p xnya_modloader_gen -- version.dll --out target/xnya_modloader_version_i686

cd target\xnya_modloader_version_i686
cargo +nightly build --target=i686-pc-windows-msvc --release
cd ..\..

cargo +nightly build --target=i686-pc-windows-msvc --release --package xnya_rallytrophy_cryptutil

mkdir dist
mkdir "dist\Rally Trophy"

copy target\xnya_modloader_version_i686\target\i686-pc-windows-msvc\release\xnya_modloader.dll "dist\Rally Trophy\version.dll"
copy target\xnya_modloader_version_i686\target\xnya_modloader.toml "dist\Rally Trophy"
copy target\i686-pc-windows-msvc\release\xnya_rallytrophy_cryptutil.dll "dist\Rally Trophy"
copy target\xnya_rallytrophy_cryptutil.toml "dist\Rally Trophy"

echo Build complete