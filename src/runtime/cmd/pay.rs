use std::collections::hash_map::Entry;

use crate::{
    Runtime,
    aux::Owned,
    ext::{Balance, Money},
    runtime::model::Pair,
};

use super::model::Dir;

/// Move money from *source* to *target*.
///
/// This modifies the [`super::Balance`] ***negatively***!
/// The balance denotes how much *source* owes the *target*,
/// which is how much *source* would need to pay *target*
/// to be on 0 again.
///
/// For example, `A` paying 1€ to `B` means
/// the balance from `A` to `B` will be -1€.
#[derive(Owned!)]
pub struct Pay {
    pub amount: Money,
    pub who: Dir,
}

impl Runtime {
    pub fn pay(&mut self, cmd: Pay) {
        // the Pair conversion may reorder
        // (so different orders still get the same balance)
        // if the order is inequal, add
        // if the order is equal, subtract
        // (other order would be possible but imply negation in the `balance` cmd)

        let mut value: Balance = cmd.amount.into();
        if !cmd.who.would_reorder() {
            value.flip();
        }

        let lookup: Pair = cmd.who.clone().into();
        match self.state.balances.entry(lookup) {
            Entry::Occupied(mut entry) => *entry.get_mut() += value,
            Entry::Vacant(entry) => {
                entry.insert(value);
            }
        }
    }
}
