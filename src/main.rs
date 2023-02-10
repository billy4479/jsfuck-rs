use crate::compiler::Compiler;
use anyhow::{Ok, Result};
use clap::Parser;
use std::{fs, path::PathBuf};

mod compiler;

#[derive(Parser)]
struct Cli {
    input: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cmd = Cli::parse();

    let input = fs::read_to_string(cmd.input)?;

    let compiler = Compiler::new();
    let result = compiler.compile(input);
    match cmd.output {
        None => {
            println!("{}", result);
        }
        Some(x) => {
            if x.to_str().expect("") == "-" {
                println!("{}", result);
            }

            fs::write(x, result)?;
        }
    }
    Ok(())
}
