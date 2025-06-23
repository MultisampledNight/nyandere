use crate::{
    Runtime,
    aux::Owned,
    ext::Money,
    runtime::model::{Dir, Split},
};

#[derive(Owned!)]
pub struct Deliver {
    pub who: Dir,
    pub price: Money,
    pub split: Split,
}

impl Runtime {
    pub fn deliver(&mut self, Deliver { who, price, split }: Deliver) {
        // at the moment a delivery has no difference to a payment
        // the logic of finding the price is handled in the repr
        // since possession is not modelled nor deliveries/payments/purchases tracked
        // TODO: track them all ^

        let (_source_supply, target_supply) = split.split(price);

        self.pay(super::Pay {
            amount: target_supply,
            who,
        });
    }
}
