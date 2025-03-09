use super::actor::Entity;

use crate::aux::{Common, Owned};

/// Do something that makes, modifies or reads.
#[derive(Common!)]
pub enum Command {
    Create,
    Pay,
    Deliver,
    Purchase,
    Stats,
    Balance,
}

/// Directed edge between 2 [`Entity`]ies.
#[derive(Owned!)]
pub struct Dir {
    pub from: Entity,
    pub to: Entity,
}
