//! Library defining all components necessary to compile Sente programs to WebAssembly

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(grammar);
pub mod ast;
pub mod emitter;
pub mod services;
pub mod wasm;

use std::io::{BufReader, BufWriter, Read, Write};
/// Compiles Sente code read from reader and writes the result with writer.
/// Prints compile errors to the standard output.
/// Calls [grammar::ProgramParser::parse] and [emitter::emit].
pub fn compile<R, W>(
    reader: &mut BufReader<R>,
    writer: &mut BufWriter<W>,
) -> Result<(), std::io::Error>
where
    R: Read,
    W: Write,
{
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    match grammar::ProgramParser::new().parse(&input) {
        Ok(parsed_program) => match emitter::emit(parsed_program) {
            Ok(emitted_code) => {
                writer.write_all(&emitted_code)?;

                Ok(())
            }
            Err(error) => {
                println!("Compiler error: {}", error);
                Ok(())
            }
        },
        Err(error) => {
            println!("{}", error);
            Ok(())
        }
    }
}
