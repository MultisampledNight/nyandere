#import "prelude.typ": *

#let field(n) = (0, -1.5 * n)
#let cell = (x: 10, y: -10)
#let linespace = 1.5

#let tables(..it, columns: 3, accents: (:)) = {
  it.named()
    .pairs()
    .enumerate()
    .map(((idx, (table, fields))) => {
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
    }.to-dict())
    .join()
}
#let dt = gamut.sample(65%)

#let schema = gfx.diagram(
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
    entity: (
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
      entity-id: entity,
      delivery-id: delivery,
      payment-id: payment,

      instance_of: concept,

      of: object,
      during: session,
      payment: payment,

      from: entity,
      to: entity,

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
          ("session-id", 45%, "entity-id"),
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

      entity-id: line(
        (rel: (4, 0), to: "entity-id"),
        corner("payment-from"),
        corner("payment-to"),
      ),
    )
  }
)

// assuming it's compiled standalone for some reason
#show: template.gfx
#schema
