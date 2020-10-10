#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(grammar);
mod ast;
mod emitter;

use std::io::{BufReader, BufWriter, Read, Write};
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

    let parsed_result = grammar::ExprParser::new().parse(&input);

    println!("{:?}", parsed_result);

    let emitted_code = emitter::emit();
    writer.write_all(&emitted_code)?;

    Ok(())
}
