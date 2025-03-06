#import "prelude.typ": *
#import "gfx.typ"

#let cmds = table.with(
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
    [Command],
    [What does it do],
  ),
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
  keywords
    .remove("bold")
    .map(term => (term, strong))
  keywords.pairs()
}.to-dict()

#show: note.with(
  title: "Nyandere",
  subtitle: "we heard you like category theory so we abstracted goods into objects and their value into a separate category and traded that instead",
  keywords: keywords,
)
#show heading.where(level: 1): (
  it => pagebreak(weak: true) + it
)

Keeping track of purchases and balances between entities.
No warranty.

= Idea

#let (bal, pay, deliver) = ("bal", "pay", "deliver").map(math.op)

== Notation

/ $a -> b$: Entity $a$ *to* entity $b$.

/ $a ->^o b$: Entity $a$ *gives* object $o$ *to* entity $b$.

== Terms

/ Entity $e$:
  An actor in this system
  that can make and receive
  payments and deliveries.
  <entity>

/ Session $s$:
  One time period during which
  any count of payments and deliveries
  are made.
  <session>

=== Payment of value

/ Payment $pay_i (a -> b)$:
  The $i$-th value transfer from $a$ to $b$.
  <payment>
  - The value in one payment is always $>= 0$.
  - One entity can make any number of payments.
    - Including multiple ones to the same entity.

/ Balance $bal(a -> b)$:
  How much $a$ owes $b$.
  <balance>
  - Equivalently: How much $a$ would need to pay to $b$ in order for the payments to each other to zero out.
  - May be negative.

  #define[$
  bal(a -> b)
  := sum_i pay_i (b -> a)
  - sum_i pay_i (a -> b)
  $]
  #propose[$
  bal(a -> b) = - bal(b -> a)
  $]

=== Giving things around

/ Object $o$:
  One physical body that can be given around.
  <object>
  - Assumed to be discretely counted.
  - May be an instance of a concept.
    - The concept can be used as index then,
      e.g. $o_c$.

/ Concept $c$:
  The general idea of a product.
  <concept>
  - Has a GTIN, a name, the works.
  - Example: When one says
    "I'm going to buy an Izeps",
    one usually means buying an *object*
    that is an *instance*
    of the *concept* "Izeps".

/ Delivery $deliver(a ->^o b)$:
  Transfer of $o$ from $a$ to $b$
  implying a payment $pay(a -> b)$
  of the value $a$ expects from $b$ for $o$.
  <delivery>

  - Note that the payment
    is direct part of the delivery.
  - If $o$ is a _gift_,
    the associated payment
    has the value $0$.
  - Possession isn't modelled!
    - So theoretically,
      $a$ *could* deliver the same $o$
      to both entities $b, c$
      separately.


= Usage

- A REPL is started via the `nyandere` command
- All REPL commands can also be
  specified on the command line instead
  - Though since no environmental context exists,
    it has to be all specified ad-hoc

== Meta-info

=== Context

- To avoid repetition, all commands happen in a context
- This allows running many commands with similar parameters
- Essentially just a dictionary

=== Session control

- Automatically handled
- Opening a new REPL starts a new session,
  closing one ends it
- All commands like purchases and deliveries
  are atomic via transactions

== Commands

=== Context interactors

- Allow modifying and setting context variables
- Can also be stacked in one line
  by just writing them after each other
  - In case of duplicates, the last one takes precedence

==== Set <set>

- Arguments are listed in brackets `[]`
  with their expected type inside them
- The command keywords themselves are called *keys*

#cmds(
  `from [entity]`,
  [What entity is the object acquired from? \
  Defaults to the `store` entity.],

  `by [entity]`,
  [Who purchases the object?],

  `of [object/concept]`,
  [What object is bought, or which concept is instantiated and then bought?],

  `for [entity]`,
  [Who will receive the object? \
  Defaults to `by`.],

  `at [price]`,
  [For how much is the object bought from `from`?
  Defaults to the concept's default price if possible.],

  `resell [price]`,
  [How much does `by` want from `for`?
  Defaults to `at`.
  Ignored if `by == for`.],

  `range
  [datetime]
  [datetime]`,
  [Over what range the statistics operate.],
)

==== Unset

#cmds(`unset [key]`)[
  - Forgets the context set for this key
  - The default will be used instead, if any
  - If there's no default, it'll need to be supplied again

  #hint[
  So to unset `for`,
  one would write `unset for`.
  ]
]

==== Get

Introspect the current context.

#cmds(
  `context`,
  [Print all explicitly set context keys.],

  `get [key]`,
  [Print just the value currently set for the key,
  or its computed default,
  or note that it is unset.],
)

=== Actions

- Do something with the given context
- Note that the context is left untouched
- The only commands that can
  interact with permanent state
- Context being optional or not is denoted via the delimiter kind:
  - *Necessary* has angle brackets (`<>`)
  - *Optional* has parentheses (`()`)
    - Including conditionally optional ones
      #fade[(ones that change in optionality depending on other context)]


#cmds(
  `purchase
  <by> <of>
  (from) (for)
  (at) (resell)`,
  [#todo[
    `from` delivers `of` to `by` at `at`,
    `by` pays `at` to `from`,
    `by` delivers `of` to `for` at `at`]],

  `pay
  <at>
  <from> <to>`,
  [Pays `at` from `from` to `to`.],
  
  `deliver
  <of>
  <from> <to>
  (at)`,
  [Delivers `of` from `from` to `to` while also paying `at` from `from` to `to`.],

  `gift
  <of>
  <from> <to>`,
  [Alias to `deliver <of> <from> <to> at 0`.],

  `stats (range)`,
  [
  - Default for `range`: Last 30 days
  - Prints
    - Money spent in total
    - Count of purchases
    - Highest money spent in one purchase
    - Average money spent per purchase
    - Average products per purchase
  ],
)

== ABNF

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
  let indices = range(it.len())
    .map(idx => fade($#(idx + 1) &&$))
    .join()
  let digits = chunks-from-right(it.clusters())
    .map(chunk => chunk
      .map(digit => $#digit &&$)
      .join(h(1em))
    )
    .join(h(1.5em))

  block(width: 100%, $
  #fade("idx:") #h(0.75em) &&#indices \
  "code:" #h(0.75em) &&#digits
  $)
}

- GTIN-13 as found on the barcode:

  #barcode(4002846034504)

- Mapped to GTIN-14:

  #barcode("04002846034504")

= Resources and references

- https://en.wikipedia.org/wiki/Global_Trade_Item_Number
- https://www.sea-ql.org/sea-orm-tutorial
- https://www.sea-ql.org/sea-orm-cookbook/007-run-migration-at-app-startup.html

