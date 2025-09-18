use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "hexplore")]
#[command(version, about)]
pub struct Args {
    #[arg(help = "The file that should be opened")]
    pub file: String,
    #[arg(short, long, default_value_t = 16)]
    pub align: usize,
}
