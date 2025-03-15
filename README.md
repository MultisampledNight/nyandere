# nyandere

Finally figure out how much everyone owes each other.

## Installation

If [Rust] is already installed,
just write `cargo install nyandere`!
Otherwise check out [the latest release].

## Usage

Much like one would write an R, Julia or Python script
to describe the processing some data,
one can write a nyandere script
like a receipt!

The syntax is inspired by [SQL].
It is case-sensitive, though.

Any script generally consists of 3 phases:
**creation**,
**transfer** and
**analysis**.

### Creation

An **actor**
is a stateful participant
in an interaction.
Each actor needs to be *created*
before it can be used,
this avoids errors due to typos or the works.

An actor is one of the following kinds:

- Entity: Holds a balance to other entities and
    can make deliveries, can be created via `create entity <name>`
- Concept: An off-the-shelf somewhat standardized product
    with a name, optionally a default price and optionally a [GTIN].
    Can be created via `create concept <name> (price <price>) (gtin <gtin>)`
    - About the GTIN
        - Is a base10 number with no leading zeroes between 8 and 14 digits long
            (both inclusive)
        - Thought for use with a barcode scanner
        - There's cheap used ones connectable via USB
- Object: One physical object, possibly an instance of a concept.
    Can be created via `create object <name> (parent <concept>)`
- Product: A concept or object.

For example,
this would create 2 entities `A`, `B` and
a concept `Multikey`
with the price of 1.70€ and
the GTIN `10000000`,
as well as an object `thing` with `Multikey` as parent concept:

```nyan
create entity A
create entity B
create concept Multikey price 1.70€ gtin 10000000
create object thing parent Multikey
```

A few more caveats:

- An object inherits its default price from its parent, if any.
- One can refer to a concept by its name or GTIN
    iff it's been specified at creation.
- Names and GTINs can be *shadowed*.
    - For example, one can create 2 concepts with the name `cotton`
        after each other, but only the last created one
        is accessible by that name.
    - However, the shadowed actors still exist. They aren't replaced.
        Existing transfers and parent relationships aren't changed
        by shadowing.
- Specifying products by name looks up objects before concepts.

### Transfer

A transfer is one of:

- **Payment** of money from an entity to an entity via
    `pay <money> from <entity> to <entity>`
- **Delivery** of a product from an entity to an entity via
    `deliver <product> (price <money>) from <entity> to <entity>`
    - The price expresses
        what the source entity *expects back*
        from the target entity
    - It is a value debit expected back at some point, in a way
    - Has to be specified only if the product doesn't have a default price

### Analysis

Look at what happened from a larger point of view.

- The **balance** between two entities is how much they owe each other.
    It is the deliveries (with expected values) and payments to each other
    summed up:
    `balance from <entity> to <entity>`

## License

Please do note that
while I do hope this is useful to some,
this is a project
created in my personal free time.
I am not a professional,
nor is this a product I am selling.

Licensed under either of

- Apache License, Version 2.0
    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[Rust]: https://www.rust-lang.org/
[SQL]: https://en.wikipedia.org/wiki/SQL

[GTIN]: https://en.wikipedia.org/wiki/Global_Trade_Item_Number
[the latest release]: https://github.com/MultisampledNight/nyandere/releases/latest
