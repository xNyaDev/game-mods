@echo off

if not exist target\Ultimate-ASI-Loader.zip (
    powershell -Command "& {(New-Object Net.WebClient).DownloadFile('https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/download/v4.92/Ultimate-ASI-Loader.zip', 'target/Ultimate-ASI-Loader.zip')}"
    powershell -Command "Expand-Archive -LiteralPath 'target/Ultimate-ASI-Loader.zip' -DestinationPath 'target/Ultimate-ASI-Loader'"
    REM powershell -Command "& {(New-Object Net.WebClient).DownloadFile('https://github.com/ThirteenAG/Ultimate-ASI-Loader/releases/download/v4.92/Ultimate-ASI-Loader_x64.zip', 'target/Ultimate-ASI-Loader_x64.zip')}"
    REM powershell -Command "Expand-Archive -LiteralPath 'target/Ultimate-ASI-Loader_x64.zip' -DestinationPath 'target/Ultimate-ASI-Loader_x64'"
)

cargo +nightly build --target=i686-pc-windows-msvc --release --package xnya_rallytrophy_cryptutil

mkdir dist
mkdir "dist\Rally Trophy"

copy target\Ultimate-ASI-Loader\dinput8.dll "dist\Rally Trophy\version.dll"
copy target\i686-pc-windows-msvc\release\xnya_rallytrophy_cryptutil.dll "dist\Rally Trophy\xnya_rallytrophy_cryptutil.asi"

echo Build complete