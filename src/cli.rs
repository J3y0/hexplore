use clap::Parser;

fn blocksize_in_range(s: &str) -> Result<u16, String> {
    let blocksize = s
        .parse()
        .map_err(|_| format!("'{s}' is not a valid value for blocksize"))?;
    if blocksize == 0 {
        Err(String::from(
            "blocksize should not be 0 but strictly positive",
        ))
    } else {
        Ok(blocksize)
    }
}

#[derive(Parser, Debug)]
#[command(name = "hexplore")]
#[command(version, about)]
pub struct Args {
    #[arg(help = "The file that should be opened")]
    pub file: String,
    #[arg(short, long, help = "The number of bytes per block")]
    #[arg(value_parser = blocksize_in_range)]
    pub blocksize: Option<u16>,
}
