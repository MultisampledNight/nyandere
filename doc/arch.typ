#import "prelude.typ": *
#import "gfx.typ"

#let detail(target: [Command], ..args) = table(
  columns: (auto, 1fr),
  align: left + horizon,
  fill: (x, y) => if x == 0 and y > 0 { halcyon.bg },
  stroke: (x, y) => {
    let grey = gamut.sample(30%)
    if x == 0 { (right: grey) }
    if y > 0 { (top: grey) }
  },
  inset: 0.75em,
  table.header(
    strong(target),
    strong[Description],
  ),
  ..args,
)

#let plurals = (
  entities: entity,
  deliveries: delivery,
  payments: payment,
)
#let keywords = (
  paint: (accents + plurals),
  bold: (
    "context",
    "command",
    "context interactor",
    "context interactors",
    "action",
    "actions",
  ),
  GTIN: link.with(<gtin>),
)
#let keywords = {
  keywords
    .remove("paint")
    .pairs()
    .map(((name, accent)) => (name, text.with(accent)))
  keywords.remove("bold").map(term => (term, strong))
  keywords.pairs()
}.to-dict()

#show: note.with(
  title: "Nyandere",
  desc: "Scripting language for keeping track of purchases and balances between entities.",
  notice: "No warranty, see the license.",
  alt-title: "we heard you like category theory so we categorized money",
  keywords: keywords,
)
#show heading.where(level: 1): (
  it => pagebreak(weak: true) + it
)

= Idea

== Goals

Build a scripting language that:

+ Allows tracking one's purchases over time
  - For personal use
  - For club purposes
+ Is readable for non-technically-inclined users
+ Can be used to understand
  how sums came to be

== Parts

=== Introduce

/ Actor: Something or someone participating here
  that may participate
  in actions or analysis and persists beyond them.
  - An entity, object or concept.

/ Entity $e$:
  An actor in this system
  that can make and receive
  payments and deliveries.

/ Object $o$:
  One physical body that can be given around.
  - Assumed to be discretely counted.
  - May be an instance of a concept.
    - The concept can be used as index then,
      e.g. $o_c$.

/ Concept $c$:
  The general idea of a product.
  - Has a GTIN, a name, the works.
  - Example: When one says
    "I'm going to buy an Izeps",
    one usually means buying an *object*
    that is an *instance*
    of the *concept* "Izeps".

=== Track

Here, $a, b$ are entities, $o$ is an object.

/ Action:
  Something that can be performed, possibly repeatedly,
  affecting one or multiple actors.
  - Payments, delivery or combinations of those.

/ Session $s$:
  A time period during which
  any number of actions
  are made.

/ Payment:
  Money transfer from $a$ to $b$.
  - Always measured in whole cents.
  - The value in one payment is always $>= 0$.
  - One entity can make any number of payments.
    - Including multiple ones to the same entity.

/ Delivery:
  Transfer of $o$ from $a$ to $b$.
  - Every delivery has one associated payment
    from $a$ to $b$.
    Yes, that order is right.
    - It represents the value that $a$ expects back from
      $b$.
    - If $o$ is a _gift_,
      the associated payment
      has the value $0$.

  - Possession isn't modelled!
    - So theoretically,
      $a$ *could* deliver the same $o$
      to both entities $b, c$
      separately.

=== Understand

/ Analysis:
  A read-only operation
  over previous actions
  yielding insight on them.

/ Balance:
  How much $a$ owes $b$.
  <balance>
  - Equivalently: How much $a$ would need to pay to $b$ in order for the payments to each other to zero out.
  - May be negative.


= Implementation

- Is a proper scripting language intended to be run as such
- A REPL can also be started via the `nyandere` command
- Comments are begun with `#`
- Statements are commands,
  - Terminated by a newline

== Types

- The syntax for the parameter placeholders below is
  `<name:type>`
  - `name` refers to the parameter in the description text
  - `type` is one of
    `ident`,
    `money`,
    `product`,
    `range`,
    `entity`,
    `object` or
    `concept`,
    explained below
- `<name>` can also be used if the name for this parameter
  is equal to the type
  - I.e. it expands to `<name:name>`

#detail(
  target: [Type],
  `ident`,
  [
    Textually names something.
    Must start with an alphabetic
    character,
    then follow any number of alnum,
    `-`, `_` or `/` characters.

    - Example: `trans/4-20`
  ],

  `money`,
  [
    Constructed value for goods.
    The `value` can be specified in cents (e.g. `420 ct`)
    or euros in decimal form (e.g. `4.20 eur`).

    `ct` or `eur` may be omitted, if so, cents are assumed.
  ],

  `product`,
  [
    A created `gtin`, `concept` or `object`.
    If a `concept`, they may be instantiated into
    anonymous `object`s by the command.
  ],

  `gtin`,
  [
    Global Item Trade Number
    usually found as a barcode on products
    in stores.
    Consists of 8, 12, 13 or 14 digits.

    - Example: `12345678`
  ],

  `range`,
  [
    Finite temporal interval.
    Start and end are inclusive and
    datetimes in RFC 3339 style
    with `T` or whitespace as date/time separator.
    Start and end of the interval are delimited by two dots
    (`..`).

    - Example: `2025-03-01..2025-03-07`
  ],

  `entity`,
  [
    `ident` that has been previously `create entity`'d.
  ],

  `object`,
  [
    `ident` that has been previously `create object`'d.
  ],

  `concept`,
  [
    `ident` that has been previously `create concept`'d,
    or a `gtin` referring to one that is.
  ],
)

See the full syntax description in @syntax
on the specifics of how to write them.

== Commands

- Parameters have placeholder names in `<>`
- `()` denotes that something is optional

=== Creation

#detail(
  `create entity <name>`,
  [Registers a new entity.],

  `create object <name>
  (instance-of <concept>)`,
  [Registers a new object.
    If `concept`
    is specified, that's what it'll be an instance of.

    Note that you'll basically never use this,
    chances are you actually want to create a concept instead
    .
  ],

  `create concept <name>
  (price <price>)
  (gtin <gtin>)`,
  [Registers a new concept.
    Has as default `price`,
    if it's unset it will need to be specified every time.
    If `gtin` is set, you can use it as
    alias equivalent for
    products.
  ],
)

=== Actions

#detail(
  `pay <value>
  from <from>
  to <to>`,
  [Transfers `value` money from `from` to `to`.
  ],

  `deliver <product>
  (price <value>)
  from <from>
  to <to>`,
  [...],

  `purchase <product>
  (price <value>)
  from <from>
  to <to>`,
  [...],
)

=== Analysis

#detail(
  `stats (<range>)`,
  [...],

  `balance
  from <from>
  to <to>`,
  [...],
)


== Syntax <syntax>

In ABNF:

#raw(read("syntax.abnf"), lang: "abnf", block: true)


= Database

#gfx.schema


= Example

Assuming
entities $t, u, k$,
concept $c$,
object $o$,
sessions $s_n$,
payments $p_n$ and
deliveries $d_n$.
Price function for object $o$ is $P(o)$.

== Physically

+ $t$ buys $o$ at store $k$
+ $t$ gives $o$ to $u$
+ At some later point in time,
  $u$ gives $P(o)$ to $t$

== Technically

=== Via UI

```
nyandere
> entity t from k
> purchase c for u
```

=== Behind the scenes

+ Creation of users $t, u, k$
+ During new session $s_1$
  + Create $c$
  + Create $o$ as instance of $c$
  + $p_1$: Pay $P(o)$ from $k$ to $t$
  + $d_1$: Deliver $o$ with $p_1$
  + $p_2$: Pay $P(o)$ from $t$ to $k$
+ During new session $s_2$
  + $p_3$: Pay $P(o)$ from $t$ to $u$
  + $d_2$: Deliver $o$ with $p_2$
+ Sometime later, during new session $s_3$
  + $p_4$: Pay $P(o)$ from $u$ to $t$


= Possible future extensions

- Bottle deposits
- Different shops and their default prices each
  - Potentially even linked to and queryable with Wikidata??
    That'd be amazing
    (though probably also implying a lot of legal trouble)
- Expiration dates
- Consumption of objects
  - And tracking who owns which object

= Glossary

#let abbrev = fxfirst.with(fx: text.with(duality.green))

== #abbrev("Global Item Trade Number") <gtin>

=== Context

- Typically found on barcodes and the works
- Allows identifying a product worldwide
- Also encompass EANs and book ISBNs
- Unfortunately there's no easy DB to download to give them more information
  - API access is gated behind GS1
    who want money, what else

=== Technical

- 4 types: GTIN-{14,13,12,8}
  - Suffix defining the digit count
- All types can be mapped to GTIN-14
  - By prefixing with zeroes
    until 14 digits in length in total

==== Digit semantics

Based on their indices.
Starting from 1,
for GTIN-14:

/ ${1}$: Indicator
  - ${0}$ Item itself without packaging
  - $[1, 8]$ Packaging level
    - No global consensus on which number means what
  - ${9}$ Variable measure
/ $[2, 13]$: Item identifier
  - Company prefix + company item
  - Prefix can have different lengths
/ ${14}$: Check digit
  #question[
    Can be calculated from the others
    via adding, then modulo 10?
  ]


==== Example: A certain mate

#let barcode(it) = {
  it = str(it)
  let chunks-from-right(it, n: 4) = it.rev().chunks(n).rev().map(array.rev)

  // whether the spacing is done on the indices or digits is irrelevant
  // due to the alignment bringing them on the same level anyway
  // so i decided for the digits
  let indices = range(it.len()).map(idx => fade($#(idx + 1) &&$)).join()
  let digits = chunks-from-right(it.clusters())
    .map(chunk => chunk.map(digit => $#digit &&$).join(h(1em)))
    .join(h(1.5em))

  block(
    width: 100%,
    $
      #fade("idx:") #h(0.75em) &&#indices \
  "code:" #h(0.75em) &&#digits
    $,
  )
}

- GTIN-13 as found on the barcode:

  #barcode(4002846034504)

- Mapped to GTIN-14:

  #barcode("04002846034504")

= Resources and references

- https://en.wikipedia.org/wiki/Global_Trade_Item_Number
- https://www.sea-ql.org/sea-orm-tutorial
- https://www.sea-ql.org/sea-orm-cookbook/007-run-migration-at-app-startup.html

