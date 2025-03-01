#import "@preview/flow:0.3.0": *

#let accents = duality.values().slice(2)
#let accents = (
  "concept",
  "object",
  "entity",
  "session",
  "delivery",
  "payment"
).zip(accents).to-dict()
#let (
  concept,
  object,
  entity,
  session,
  delivery,
  payment,
) = accents
