@echo off

cargo +stable-i686-pc-windows-gnu build --manifest-path rust\pn_rust_dll\Cargo.toml --release
copy rust\pn_rust_dll\target\release\pn_rust_dll.dll extensions\PNRust\pn_rust.dll
