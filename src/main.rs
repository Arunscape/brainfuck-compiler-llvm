#![warn(clippy::all)]

use clap::Arg;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::targets::{InitializationConfig, Target};
use inkwell::OptimizationLevel;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("ABC - Arun's Brainfuck Compiler")
        .version("0.1")
        .author("Arun Woosaree")
        .about("Compiles Brainfuck to LLVM")
        .arg(Arg::with_name("INPUT").required(true))
        .arg(
            Arg::with_name("output")
                .short("o")
                .value_name("OUTPUT_FILE")
                .takes_value(true),
        )
        .get_matches();

    let output = matches.value_of("output");

    // required argument, so I can unwrap
    let input = matches.value_of("INPUT").unwrap();
    let input = fs::read_to_string(input)?;
    println!("{}", input);
    Ok(())
}
