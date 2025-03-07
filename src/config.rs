use clap::Parser;

#[derive(Parser, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[command(version, about, author)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

pub fn cli() -> Config {
    Parser::parse()
}
