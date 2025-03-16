use std::collections::hash_map::Entry;

use crate::{
    Runtime,
    aux::Owned,
    ext::{Balance, Money},
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
        let mut value: Balance = cmd.amount.into();
        let key = value.take_order(cmd.who);

        match self.state.balances.entry(key) {
            Entry::Occupied(mut entry) => *entry.get_mut() += value,
            Entry::Vacant(entry) => {
                entry.insert(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Set, eval, ext::Integer};

    #[test]
    fn basic() {
        let rt = eval(
            "
            create entity A
            create entity B
            create entity C

            pay 1€ from A to B
            pay 5€ from B to A
            pay 3€ from A to C
            ",
        )
        .unwrap();

        let s = |lit: &str| lit.to_string();
        let i = Integer::from;

        assert_eq!(
            rt.to_state()
                .balances
                .into_iter()
                .map(|(pair, bal)| {
                    (
                        pair.into_iter()
                            .map(|e| e.name().to_owned())
                            .collect::<Vec<_>>(),
                        bal.0,
                    )
                })
                .collect::<Set<_>>(),
            [
                (vec![s("A"), s("B")], i(400)),
                (vec![s("A"), s("C")], i(-300))
            ]
            .into_iter()
            .collect::<Set<_>>(),
        );
    }
}
