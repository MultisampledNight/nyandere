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

```shell
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

= Future extensions

- Different users
- Bottle deposits
- Different shops and their default prices each
  - Potentially even linked to and queryable with Wikidata??
    That'd be amazing
    (though probably also implying a lot of legal trouble)

= Resources and references

- https://en.wikipedia.org/wiki/Global_Trade_Item_Number
