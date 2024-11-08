#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
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
