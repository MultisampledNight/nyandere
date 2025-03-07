use crate::ext::Gtin;

pub struct Script(pub Vec<Stmt>);

/// Something that can be done.
pub enum Stmt {
    Command(Command),
}

pub enum Command {
    Create(Create),
    Transfer(Transfer),
    Analyze(Analyze),
}

/// Commands that introduce new state.
pub struct Create {
    pub who: Actor,
}

/// Actions that do something and modify state.
pub enum Transfer {
    Pay(Pay),
    Deliver(Deliver),
    Purchase(Purchase),
}

/// Read-only commands.
pub enum Analyze {
    Stats(Stats),
    Balance(Balance),
}

/// Money transfer.
pub struct Pay {
    pub amount: Money,
    pub who: Dir,
}

/// Physical transfer implying a money transfer.
pub struct Deliver {
    pub what: Product,
    pub who: Dir,
    pub price: Option<Money>,
}

/// Delivery and payment back in one go,
/// not influencing balance hence.
pub struct Purchase {
    pub what: Product,
    pub who: Dir,
    pub price: Option<Money>,
}

pub struct Stats;

pub struct Balance {
    pub between: Dir,
}

pub enum Actor {
    Entity(Entity),
    Object(Object),
    Concept(Concept),
}

/// Holds money and resources.
pub struct Entity {
    pub name: Ident,
}

/// Can be delivered and passed around.
pub struct Object {
    pub name: Ident,
    pub instance_of: Option<Ident>,
}

pub struct Concept {
    pub name: Ident,
    pub default_price: Option<Money>,
    pub gtin: Option<Gtin>,
}

pub enum Product {
    Name(Ident),
    Id(Gtin),
}

/// Directional specification.
/// Source and recipient.
pub struct Dir {
    /// Who gives something away.
    pub from: Ident,

    /// Who receives it.
    pub to: Ident,
}

#[allow(unused)]
pub struct Ident(String);

/// Number of cents.
pub struct Money(pub u64);
