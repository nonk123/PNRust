@echo off

cargo run --manifest-path rust\generate-dll\Cargo.toml
cargo +stable-i686-pc-windows-gnu build --manifest-path rust\pn-rust-dll\Cargo.toml --release
copy target\release\pn_rust_dll.dll extensions\PNRust\pn_rust.dll
