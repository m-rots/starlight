# Starlight

A small collection of tools to help with database normalisation.

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Run `cargo install --git https://github.com/m-rots/starlight`.

## Functional dependencies

Functional dependencies can be given in any .`txt` file and should be given with the `--file` option.
The file should follow this structure:

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

## Commands

### Calculate the cover

To calculate the cover of the set of attributes {A, B}, run:

```bash
starlight cover A,B --file deps.txt
```

*deps.txt should contain [Functional Dependencies](#functional-dependencies).*

### Check implication

To check whether a functional dependency is implied, given a set of functional dependencies, run:

```bash
starlight implication "B, C -> B" --file implication.txt
```

*deps.txt should contain [Functional Dependencies](#functional-dependencies).*