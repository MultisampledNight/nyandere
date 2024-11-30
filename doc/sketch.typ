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

- Accepts a timerange to emit statistics over
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
#let tables(..it, columns: 3, accents: (:)) = {
  it.named()
    .pairs()
    .enumerate()
    .map(((idx, (table, fields))) => array-to-dict({
      let x = calc.rem(idx, columns) * cell.x
      let y = int(idx / columns) * cell.y
      ((table, (
        pos: (x, y + linespace),
        display: [*#table*]
      )),)
      fields.enumerate().map(
        ((idx, name)) => {
          let full = table + "-" + name
          let accent = accents.at(name, default: accents.at(full, default: fg))

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
#let (concept, object, user, session, delivery, payment, ..) = duality.values().slice(2)
#let dt = gamut.sample(65%)
#gfx.diagram(
  nodes: tables(
    concept: (
      "id",
      "during",
      "gtin",
      "name",
      "default_price",
    ),
    session: (
      "id",
      "start",
      "end",
    ),
    user: (
      "id",
      "name",
    ),
    object: (
      "id",
      "instance_of",
    ),
    delivery: (
      "id",
      "during",
      "of",
      "payment",
      "at",
    ),
    payment: (
      "id",
      "during",
      "from",
      "to",
      "amount",
      "at",
    ),
    accents: (
      concept-id: concept,
      object-id: object,
      session-id: session,
      user-id: user,
      delivery-id: delivery,
      payment-id: payment,

      instance_of: concept,

      of: object,
      during: session,
      payment: payment,

      from: user,
      to: user,

      at: dt,
      start: dt,
      end: dt,
    ),
  ),
  edges: {
    import gfx.draw: *
    let line(..args) = br(..args, arrow: false)
    let dist(..args) = styled(all(..args, arrow: false))
    let corner(it) = line(vert(it), it)
    let option(..args, accent: fg) = styled(
      ..args,
      stroke: (dash: "dashed", paint: accent),
    )
    let space = 0.75
    (
      concept-id: option(line(
        ("concept-id", 45%, "session-id"),
        corner("object-instance_of"),
      ), accent: concept),
      object-id: line(
        ("object-id", 55%, "delivery-id"),
        corner("delivery-of"),
      ),
      session-id: all(
        line(
          ("session-id", 45%, "user-id"),
          corner("payment-during"),
          corner("delivery-during"),
        ),
        line(
          ("concept-id", 55%, "session-id"),
          corner("concept-during"),
        ),
      ),
      payment-id: line(
        ("delivery-id", 55%, "payment-id"),
        corner("delivery-payment"),
      ),

      user-id: line(
        (rel: (4, 0), to: "user-id"),
        corner("payment-from"),
        corner("payment-to"),
      ),
    )
  }
)

- This is a sketch. The actual column names might be different
  since we're using an ORM.
  Check the source code.
- `payment.amount` must be positive
  - A delivery implies a payment of sorts
  - So if $A$ buys something for $B$, then $A$ does a payment to $B$

= Workflow

Assuming
users $t, u, k$,
concept $c$,
object $o$,
sessions $s_1, s_2, s_3$ and
payments $p_1, p_2, p_3, p_4$.
Price function for object $o$ is $P(o)$.

- Physically
  + $t$ buys $o$ at store $k$
  + $t$ gives $o$ to $u$
  + At some later point in time,
    $u$ gives $P(o)$ to $t$

- Technically
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


= Future extensions

- Bottle deposits
- Different shops and their default prices each
  - Potentially even linked to and queryable with Wikidata??
    That'd be amazing
    (though probably also implying a lot of legal trouble)
- Expiration dates
- Consumption of objects
  - And tracking who owns which object

= Resources and references

- https://en.wikipedia.org/wiki/Global_Trade_Item_Number
- https://www.sea-ql.org/sea-orm-tutorial
- https://www.sea-ql.org/sea-orm-cookbook/007-run-migration-at-app-startup.html
