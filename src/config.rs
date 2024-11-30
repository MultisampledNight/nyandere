use clap::Parser;

#[derive(Parser, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[command(version, about, author)]
pub struct Config {
    #[arg(short, long, default_value_t = Self::default().database)]
    pub database: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // TODO: make this configurable + depend on e.g. $XDG_STATE_HOME instead
            database: "sqlite:./track.db?mode=rwc".to_string(),
        }
    }
}

pub fn cli() -> Config {
    Parser::parse()
}
