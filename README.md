# Starlight

A small collection of tools to help with database normalisation.

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Run `cargo install --git https://github.com/m-rots/starlight`.

## Calculate the cover

To calculate the cover of the set of attributes {A, B}, run:

```bash
starlight cover A,B --file deps.txt
```

Functional dependencies should be given in a file with the following structure:

```
B, E -> A, F
F -> A, B
A, D, G -> C
A -> E
D -> A, F
C, G -> A, D
B, C -> D, F
A, E, F -> B, D
```