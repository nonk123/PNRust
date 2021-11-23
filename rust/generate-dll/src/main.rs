use std::{fs, io, io::Write};

use convert_case::{Case, Casing};

fn generate_cargo_toml<T: Write>(mut w: T, modules: &Vec<String>) -> io::Result<()> {
    w.write_all(
        r#"
[package]
name = "pn_rust_dll"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
libc = "0.2"
lazy_static = "1.4"

[dependencies.pn_rust]
path = "../pn_rust"

"#
        .as_bytes(),
    )?;

    for module in modules {
        writeln!(w, "[dependencies.{}]", module)?;
        writeln!(w, "path = \"../modules/{}\"", module)?;
    }

    Ok(())
}

fn generate_glue_rs<T: Write>(mut w: T, modules: &Vec<String>) -> io::Result<()> {
    for module in modules {
        writeln!(w, "use {}::{};", module, module.to_case(Case::Pascal))?;
    }

    w.write_all(
        r"
use pn_rust::{Buffer, Context, Module, Value};

use lazy_static::lazy_static;

use std::sync::Mutex;

lazy_static! {
"
        .as_bytes(),
    )?;

    for module in modules {
        writeln!(
            w,
            "static ref {}: Mutex<{}> = Mutex::new({}::new());",
            module.to_case(Case::UpperSnake),
            module.to_case(Case::Pascal),
            module.to_case(Case::Pascal),
        )?;
    }

    w.write_all(
        r"
}

pub fn init(context: &mut Context) {
"
        .as_bytes(),
    )?;

    for module in modules {
        writeln!(
            w,
            "{}.lock().unwrap().init(context);",
            module.to_case(Case::UpperSnake)
        )?;
    }

    w.write_all(
        r"
}

pub fn call_function(
    context: &mut Context,
    function_name: String,
    arguments: &mut Buffer,
) -> Value {
"
        .as_bytes(),
    )?;

    for module in modules {
        writeln!(
            w,
            r#"
if let Some(result) = {}.lock().unwrap().exports(context, &function_name, arguments) {{
    return result;
}}"#,
            module.to_case(Case::UpperSnake)
        )?;
    }

    w.write_all(
        r#"
    Value::Undefined
}
"#
        .as_bytes(),
    )?;

    Ok(())
}

fn main() -> io::Result<()> {
    let cargo_toml_destination = "rust/pn_rust_dll/Cargo.toml";
    let glue_rs_destination = "rust/pn_rust_dll/src/glue.rs";

    let modules: Vec<String> = fs::read_dir("rust/modules")?
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect();

    {
        let cargo_toml = fs::File::create(cargo_toml_destination)?;
        generate_cargo_toml(cargo_toml, &modules)?;
    }

    {
        let glue_rs = fs::File::create(glue_rs_destination)?;
        generate_glue_rs(glue_rs, &modules)?;
    }

    Ok(())
}
