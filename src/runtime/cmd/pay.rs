use crate::{Runtime, aux::Owned, ext::Money};

use super::model::Dir;

/// Move money from someone to someone.
#[derive(Owned!)]
pub struct Pay {
    pub amount: Money,
    pub who: Dir,
}

impl Runtime {
    pub fn pay(&mut self, cmd: Pay) {
        todo!()
    }
}
