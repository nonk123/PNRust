@echo off

cargo +stable-i686-pc-windows-gnu build --release
copy rs_target\release\pn_rust.dll extensions\PNRust\pn_rust.dll
