use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "esprimoc")]
struct CliOptions {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn main() -> std::io::Result<()> {
    let cli_options = CliOptions::from_args();
    println!("{:#?}", cli_options);

    let output = esprimo::emitter::emit();

    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create(cli_options.output)?;
    file.write_all(&output)?;
    Ok(())
}
