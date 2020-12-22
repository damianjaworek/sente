//! Compiler for Sente language to WebAssembly.

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "sentec")]
struct CliOptions {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

/// Main function that runs whenever the compiler is launched.
/// Validates received command line arguments, opens files and calls [sente::compile]
fn main() -> std::io::Result<()> {
    let cli_options = CliOptions::from_args();

    use std::fs::File;

    let input_file = File::open(cli_options.input)?;
    let mut reader = std::io::BufReader::new(input_file);

    let output_file = File::create(cli_options.output)?;
    let mut writer = std::io::BufWriter::new(output_file);

    sente::compile(&mut reader, &mut writer)?;
    Ok(())
}
