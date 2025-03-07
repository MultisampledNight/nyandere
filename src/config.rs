use clap::Parser;

#[derive(Parser, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[command(version, about, author)]
pub struct Config {
    /// The source code to run.
    pub code: String,
}

pub fn cli() -> Config {
    Parser::parse()
}
