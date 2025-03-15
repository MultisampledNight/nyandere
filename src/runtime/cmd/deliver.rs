use crate::{Runtime, aux::Owned, ext::Money, runtime::model::Dir};

#[derive(Owned!)]
pub struct Deliver {
    pub who: Dir,
    pub price: Money,
}

impl Runtime {
    pub fn deliver(&mut self, Deliver { who, price }: Deliver) {
        // at the moment a delivery has no difference to a payment
        // the logic of finding the price is handled in the repr
        // since possession is not modelled nor deliveries/payments/purchases tracked
        // TODO: track them all ^

        self.pay(super::Pay { amount: price, who });
    }
}
