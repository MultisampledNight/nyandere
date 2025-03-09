use std::{fs, path::PathBuf};

use clap::{Args, Parser};
use eyre::{Context, Result};

#[derive(Parser, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[command(version, about, author)]
pub struct Config {
    #[command(flatten)]
    pub source: Source,
}

/// The source code to run.
#[derive(Args, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[group(required = true)] // multiple is false by default
pub struct Source {
    /// Run the code as specified on the command line.
    #[arg(short, long)]
    code: Option<String>,

    /// Read and run this source file.
    file: Option<PathBuf>,
}

impl Source {
    /// Returns the source code to run, reading if necessary.
    pub fn get(self) -> Result<String> {
        if let Some(code) = self.code {
            return Ok(code);
        }
        if let Some(file) = &self.file {
            return fs::read_to_string(file)
                .with_context(|| format!("tried to read `{}`", file.display()));
        }

        panic!(concat!(
            "configured clap to require ",
            "either inline code or source file, ",
            "however, both are empty. ",
            "this is a clap bug, please report it at our repo first though",
        ));
    }
}

pub fn cli() -> Config {
    Parser::parse()
}
