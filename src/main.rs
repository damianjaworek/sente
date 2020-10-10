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

    use std::fs::File;

    let input_file = File::open(cli_options.input)?;
    let mut reader = std::io::BufReader::new(input_file);

    let output_file = File::create(cli_options.output)?;
    let mut writer = std::io::BufWriter::new(output_file);

    esprimo::compile(&mut reader, &mut writer)?;
    Ok(())
}
