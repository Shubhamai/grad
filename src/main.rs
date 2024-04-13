use clap::Parser as ClapParser;
// TODO: Repl support ?

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path
    #[clap(value_hint = clap::ValueHint::AnyPath)]
    script: String,
}

fn main() {
    let args = Args::parse();
    let src = match std::fs::read_to_string(&args.script) {
        Ok(source) => source,
        Err(e) => panic!("Error reading file: {}", e),
    };
}
