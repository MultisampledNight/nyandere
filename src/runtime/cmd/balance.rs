use num_bigint::Sign;

use crate::{Runtime, aux::Owned, ext::Debit, runtime::model::Dir};

/// Evaluates how much `between.source` owes `between.target`.
#[derive(Owned!)]
pub struct Balance {
    pub between: Dir,
}

impl Runtime {
    pub fn balance(&self, Balance { mut between }: Balance) -> Debit {
        let bal = self.state.balance(between.clone());

        if let Sign::Minus = bal.0.sign() {
            between.flip();
        }

        Debit {
            between,
            amount: bal.abs(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Runtime, Script};

    #[test]
    fn basic() {
        let mut rt = Runtime::new();
        let script = "
            create entity A
            create entity B

            pay 1€ from A to B
            pay 4€ from B to A
            pay 1€ from A to B
        ";

        rt.run(Script::parse(script).unwrap()).unwrap();

        rt.balance(super::Balance {
            between: rt.get_dir("A", "B").unwrap(),
        });
    }
}
