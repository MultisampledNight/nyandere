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
This is intended as a help,
but it is very possible to mistakes using it or
that there's even mistakes in its design or programming.


[rust]: https://www.rust-lang.org/
[the latest release]: https://github.com/MultisampledNight/nyandere/releases/latest
