#![warn(clippy::all)]

use clap::Arg;
use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::{Linkage, Module},
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    AddressSpace, OptimizationLevel,
};
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
                .takes_value(true)
                .multiple(false)
                .number_of_values(1),
        )
        .arg(Arg::with_name("asm").help("Output in human readable assembly"))
        //.arg(
        //    Arg::with_name("target_triple")
        //        .short("tt")
        //        .value_name("TARGET_TRIPLE")
        //        .takes_value(true)
        //        .multiple(false)
        //        .number_of_values(1),
        //)
        .get_matches();

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

    //    let target_triple = if let Some(tt) = matches.values_of("target_triple") {
    //        TargetTriple::create(tt.clone().nth(0).unwrap())
    //    } else {
    //        TargetMachine::get_default_triple()
    //    };
    //

    let target_triple = TargetMachine::get_default_triple();
    let cpu = TargetMachine::get_host_cpu_name();
    let features = TargetMachine::get_host_cpu_features();

    let target = Target::from_triple(&target_triple)?;

    let target_machine = target
        .create_target_machine(
            &target_triple,
            &cpu.to_string(),
            &features.to_string(),
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .ok_or_else(|| {
            format!(
                "Could not create target machine\ntarget_triple: {}\ncpu: {}\nfeatures: {}",
                target_triple, cpu, features
            )
        })?;

    if let Some(o) = matches.value_of("output") {
        target_machine.write_to_file(&module, FileType::Object, o.as_ref())?
    } else {
        let m = context.create_module("output");
        let buffer = target_machine.write_to_memory_buffer(&module, FileType::Assembly)?;
        println!("{:?}", buffer);
    };

    Ok(())
}

// thanks to Ben Konz for this guide
// https://benkonz.github.io/building-a-brainfuck-compiler-with-rust-and-llvm/
