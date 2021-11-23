# PNRust

A Rust integration for GameMaker.

## Package Structure

The [rust](rust) directory contains all the Rust code that makes
PNRust possible. It is comprised of the following subtrees:

- [modules](rust/modules), the directory where all your PNRust modules
  are stored.
- [pn_rust](rust/pn_rust), a crate shared by all PNRust modules.
- [pn_rust_dll](rust/pn_rust_dll), a DLL that powers PNRust.
- [generate-dll](rust/generate-dll), a binary that generates the glue
  that holds the module system together.

## Getting Started

First of all, you need to copy [rs_build.bat](rs_build.bat),
[rust](rust), the `PNRust` extension, and the `pn_rust` script to
your game folder. Use "Add existing" for the latter two.

Before you can use PNRust, you need to call `rs_init` somewhere in
your game's code.

After that, create a new module in [rust/modules](rust/modules). Check
out the [example module](rust/modules/example) for more info.

Once you make any changes to the Rust code, run
[rs_build.bat](rs_build.bat). You should run it inside an open command
prompt to see any build errors.

See the next section to learn how to install the necessary Rust build
tools.

Warning: you need to have Git in your PATH for
[rs_build.bat](rs_build.bat) to work!

GML example:

```gml
rs_init()

// Assuming you've exported `my_function` from `my_great_module`.
var result = rs_call("my_function", [69])

show_debug_message("my_function: " + string(result))
```

## Rust Build Tools

First, you need to install [rustup](https://rustup.rs/). Get the
stable branch and add it to your PATH.

After that, install the `stable-i686-pc-windows-gnu` toolchain by
running this inside a command prompt:

```cmd
rustup toolchain install stable-i686-pc-windows-gnu --profile minimal
```
