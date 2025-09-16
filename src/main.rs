mod display;

use clap::Parser;
use std::fmt::Debug;
use std::fs;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(name = "hexplore")]
#[command(version, about = "Dump files with hex/ascii view.")]
struct Args {
    file: String,
    #[arg(short, long, default_value_t = 16)]
    align: usize,
}

fn main() {
    let args = Args::parse();

    match hexdump(&args.file, args.align) {
        Ok(()) => (),
        Err(e) => {
            // Gracefully terminate the program if BrokenPipe
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return;
            }
            let _ = writeln!(io::stderr(), "hexdump: error {e}");
        }
    }
}

fn hexdump(file: &str, align: usize) -> io::Result<()> {
    let content = fs::read(file)?;
    let len = content.len();
    let nb_hexdigits = count_hexdigits(len);

    for (i, chunk) in content.chunks(align).enumerate() {
        let pad = (align - chunk.len()) % align;
        writeln!(
            io::stdout(),
            "{} | {} | {} |",
            display::format_index(align * i, nb_hexdigits),
            display::format_hex(chunk, pad),
            display::format_ascii(chunk, pad)
        )?;
    }

    Ok(())
}

fn count_hexdigits(val: usize) -> usize {
    let mut i = 0;
    while val >> (4 * i) != 0 {
        i += 1;
    }

    i
}
