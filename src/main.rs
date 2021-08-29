#![warn(clippy::all)]

use clap::Arg;
use inkwell::{OptimizationLevel, builder::Builder, context::Context, execution_engine::{ExecutionEngine, JitFunction}, module::{Linkage, Module}, targets::{InitializationConfig, Target}};
use std::{error::Error, fs};

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
        .arg(Arg::with_name("asm")
             .help("Output in human readable assembly"))
        .get_matches();

    let output = matches.value_of("output");

    // required argument, so I can unwrap
    let input = matches.value_of("INPUT").unwrap();
    let input = fs::read_to_string(input)?;
    println!("{}", input);

    let context = Context::create();
    let module = context.create_module("arun_brainfuck_compiler");
    let builder = context.create_builder();

    let i64_type = context.i64_type();
    let main_fn_type = i64_type.fn_type(&[], false);
    let main_fn = module.add_function("main", main_fn_type, Some(Linkage::External));

    let basic_block = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(basic_block);



    builder.build_return(Some(&i64_type.const_zero()));
    Target::initialize_all(&InitializationConfig::default());




    Ok(())
}

// thanks to Ben Konz for this guide
// https://benkonz.github.io/building-a-brainfuck-compiler-with-rust-and-llvm/
