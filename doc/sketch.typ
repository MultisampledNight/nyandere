#import "@local/flow:0.1.0": *
#show: note.with(title: "Nyandere", subtitle: "nyaaa")

- Goal
  - Catalogize purchases
  - Distribute them money-wise

- Lowest unit: product
  - ID: #link("https://en.wikipedia.org/wiki/Global_Trade_Item_Number")[GTIN]
  - Default price
    - Can be selectively overridden
  - Count
    - Can be increased by buying
    - Can be decreased by consumption or giving to others
- Prices are usually paid by 1 person, but then later split across $n$

= Names

- Names consist of only lowercase letters and hyphens/dashes
- There's short versions of them

= Theoretical usage

== Catalogizing a shop visit

```sh
nyandere purchase
```

- Essentially a REPL stopping until a line is empty
- Asks for a product in each line
  - Has to adhere to the ABNF nonterminal `product` below
- If the ident isn't known, it asks if to register or to re-enter
  - If registering, asks for
    - Default price
    - Ident
      - If GTIN has been given, name
      - If name has been given, GTIN
      - Can be skipped by leaving it empty

```abnf
product = ident [sp count] [sp price]

ident = id / name
id = natural
; needs to start and end with a lowercase letter
; but may contain hyphens in-between
name = lowercase [*(lowercase / "-") lowercase]

count = ("x" / "*" / "times") sp natural

price = (relative / absolute) sp price-value
relative = "@" / ":" / "each"
absolute = "=" / "total"

; assume euros by default unless cents are specified
price-value = euros / cents
euros = 1*DIGIT ["." 2DIGIT] [sp ("€" / "EUR")]
cents = 1*DIGIT sp ("¢" / "ct")

lowercase = %x61-7A ; a-z
natural = 1*DIGIT
sp = *WSP
```

== Consuming something

```sh
nyandere consume <IDENT>
```

- Subtracts 1 from the count

== Statistics

```sh
nyandere stats [TIMERANGE]
```

- Optionally accepts a timerange to emit statistics over
  - If not passed, lists stats over the last 30 days
- Statistics include
  - Money spent in total
  - Count of purchases
  - Highest money spent in one purchase
  - Average money spent per purchase
  - Average products per purchase

= GTIN

- 4 different types
  - GTIN-{14,13,12,8}
- The suffix defines the length
  - And also the mapping to the GTIN-14 codes
  - They all shift to the rightmost position
    - E.g. GTIN-13 → GTIN-14 sets digit at position 1 to 0
- Book GTINs are constructed by just prefixing the 10-digit ISBN with 978 (which
  results in a GTIN-13)

== Digits

One-indexed, in the case of GTIN-14

- ${1}$ → Indicator
  - ${0}$ Item itself without packaging
  - $[1, 8]$ Packaging level
    - No global consensus on which number means what
  - ${9}$ Variable measure
- $[2, 13]$ → Item identifier
  - Company prefix + company item
  - Prefix can have different lengths
- ${14}$ → check digit
  - Can be calculated from the others

== Example: A certain mate

#let barcode(it) = {
  it = str(it)
  let chunks-from-right(it, n: 4) = it.rev().chunks(n).rev().map(array.rev)

  // whether the spacing is done on the indices or digits is irrelevant
  // due to the alignment bringing them on the same level anyway
  // so i decided for the digits
  let indices = range(it.len())
    .map(idx => dim($#(idx + 1) &&$))
    .join()
  let digits = chunks-from-right(it.clusters())
    .map(chunk => chunk
      .map(digit => $#digit &&$)
      .join(h(1em))
    )
    .join(h(1.5em))

  $
  #dim("idx:") #h(0.75em) &&#indices \
  "code:" #h(0.75em) &&#digits
  $
}

#barcode(4002846034504)

13 digits → GTIN-13

= Database

#let field(n) = (0, -1.5 * n)
#let cell = (x: 10, y: -10)
#let linespace = 1.5
#let array-to-dict(it) = it.fold((:), (a, (k, v)) => {
  a.insert(k, v)
  a
})
#let tables(..it) = {
  it.named()
    .pairs()
    .enumerate()
    .map(((idx, (table, fields))) => array-to-dict({
      let x = calc.rem(idx, 2) * cell.x
      let y = int(idx / 2) * cell.y
      ((table, (
        pos: (x, y + linespace),
        display: [*#table*]
      )),)
      fields.enumerate().map(
        ((idx, name)) => {
          let full = table + "-" + name
          let accent = (of: duality.pink, during: duality.blue)
            .at(name, default:
              (product-id: duality.pink, session-id: duality.blue)
                .at(full, default: fg)
            )

          (full, (
            pos: (x, y + -linespace * idx),
            display: text(accent, name),
            accent: accent,
          ))
        }
      )
    }))
    .join()
}
#gfx.diagram(
  nodes: tables(
    product: (
      "id",
      "gtin",
      "name",
      "default_price",
      "created_at",
    ),
    session: (
      "id",
      "start_at",
      "completed_at",
      "aborted",
    ),
    purchase: (
      "id",
      "of",
      "during",
      "count",
      "total_price",
    ),
    consumption: (
      "id",
      "of",
      "at",
      "count",
    ),
  ),
  edges: {
    import gfx.draw: *
    let center = ("purchase-of", 50%, "consumption-of")
    let space = 0.75
    (
      purchase-of: br(
        (rel: (-space, 0), to: center),
        br("consumption-of", arrow: false),
        vert("product-id"),
        "product-id",
      ),
      purchase-during: br(
        (rel: (space, 0), to: hori(center)),
        vert("session-id"),
        "session-id",
      )
    )
  }
)

= Future extensions

- Different users
- Bottle deposits
- Different shops and their default prices each
  - Potentially even linked to and queryable with Wikidata??
    That'd be amazing
    (though probably also implying a lot of legal trouble)
- Expiration dates

= Resources and references

- https://en.wikipedia.org/wiki/Global_Trade_Item_Number
