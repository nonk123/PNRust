# PNRust

A Rust integration for GameMaker.

## Getting Started

First of all, you need to copy [rs_build.bat](rs_build.bat),
[rs_src](rs_src), [.cargo](cargo), [Cargo.toml](Cargo.toml), the
`PNRust` extension, and the `pn_rust` script to your game folder. Use
"Add existing" for the latter two.

Before you can use PNRust, you need to call `rs_init` somewhere in
your game's code.

After that, modify [exports.rs](rs_src/exports.rs) (which see) to
export global functions to GML.

A complete example would look like this:

```rust
use crate::buffer::Buffer;
use crate::value::Value;
use crate::Context;

pub fn init_exports(context: &mut Context) {
    context.export("my_function", my_function);
}

pub fn my_function(context: &mut Context, args: &mut Buffer) -> Value {
    Value::Real(420.69)
}
```

Once you make any changes to the Rust code, run
[rs_build.bat](rs_build.bat). You should run it inside an open command
prompt to see any build errors.

See the next section to learn how to install the necessary Rust build
tools.

You can then use `my_function` inside GML.

If all went well, the following code should print: `my_function:
420.69`:

```gml
rs_init()
show_debug_message("my_function: " + string(my_function()))
```

## Rust Build Tools

First, you need to install [rustup](https://rustup.rs/). Get the
stable branch and add it to your PATH.

After that, install the `stable-i686-pc-windows-gnu` toolchain by
running this inside a command prompt:

```cmd
rustup toolchain install stable-i686-pc-windows-gnu --profile minimal
```
