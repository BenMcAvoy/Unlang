#set text(font: "Liberation Sans", lang: "en")
#set document(author: "Ben Mcavoy", title: "Docs")
#set page(numbering: "1", number-align: center)
#show math.equation: set text(weight: 400)
#set par(justify: true)

#align(center)[
  #block(text(weight: 700, 1.75em, "Grammar"))
]

== Productions
- `[exit]` $arrow.r$ exit(`[expr]`);
- `[expr]` $arrow.r$ `IntLit`

== Formatting
- `[expr]` $arrow$ Expression
- `[...]`  $arrow$ Production
- `...`    $arrow$ Token

#linebreak()

#let cell = rect.with(
  inset: 8pt,
  //fill: rgb("e4e5ea"),
  fill: rgb("fff"),
  width: 100%,
  radius: 6pt
)

== Example trees.
#set align(left)
#grid(
  columns: (25%, 25%),
  rows: (20%, auto),
  gutter: 3pt,
  cell(height: 100%)[
    #set align(center)
    #rect[`exit()`]
    $arrow.b$
    #rect[`[expr]`]
    $arrow.b$
    #rect[`[+]`]
    $arrow.b$

    #grid(
      columns: (30%, 30%, 30%),
      rows: (100%, 100%),

      cell(height: 100%)[
        #rect[
          `1`
        ]
      ],
      cell(height: 100%)[
        and
      ],
      cell(height: 100%)[
        #rect[2]
      ]
    )
  ],
  cell(height: 100%)[
    #set align(center)
    #rect[`exit()`]
    $arrow.b$
    #rect[`[expr]`]
    $arrow.b$
    #rect[`1`]
  ],
)
