# nyandere

i love cats.
but i do not love keeping track of how much i owe someone
and how much they owe me.
so i made this to keep track of it for me.

## installation

if you already have [rust] installed,
just write `cargo install nyandere`.
otherwise check out [the latest release].

## usage

use `nyandere purchase` if you bought something.
each line is a product you've bought,
following the ABNF nonterminal `product`
described in [`parse`](./src/parse.rs).

put short: it's the identifier,
optionally `*` how many times you bought it,
optionally `@` at which cost of one product
(or `=` for the cost in total).

if the identifier isn't known yet,
it'll ask you if you want to register it
so you can set a default price.

the identifier can be

- a [GTIN] (the barcode you're finding on a lot of products)
- a custom set name (consisting only of lowercase alphabetic characters and numbers)

suggestion: get a barcode scanner
for inputting the GTINs.

## License

Please note that
while I do hope this is useful to some,
there's **no warranty whatsoever**.

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


[GTIN]: https://en.wikipedia.org/wiki/Global_Trade_Item_Number
[rust]: https://www.rust-lang.org/
[the latest release]: https://github.com/MultisampledNight/nyandere/releases/latest
