@echo off

cargo run --manifest-path rust\generate-dll\Cargo.toml

git update-index --assume-unchanged rust\pn_rust_dll\Cargo.toml
git update-index --assume-unchanged rust\pn_rust_dll\src\glue.rs

cargo +stable-i686-pc-windows-gnu build --manifest-path rust\pn_rust_dll\Cargo.toml --release

copy target\release\pn_rust_dll.dll extensions\PNRust\pn_rust.dll
