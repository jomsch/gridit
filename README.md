# Gridit

A 2D grid library utilizing the fun of iterators.


# Examples

Run the examples as following:  

## Game Of Life
```
cargo run --example gameoflife
```

## Board
```
cargo run --example board
```

The piece with the `T` in it is a `Blocker` it can not beat any pieces and moves by teleporting
to a friendly piece.
The `Giraffe` is a blank piece for testing purposes. If you want to quickly test this library
i recommend to implement some movement pattern for the [Giraffe](./examples/board/piece/giraffe.rs) in `fn possible_moves`.
The movement patterns of all pieces can be found [here](./examples/board/pieces).


# Asset License
Assets found in [resources](./resources/) are licensed under [CC-BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/legalcode).
These assets can be found [here](https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces).
Thanks and credit to:
* Colin M.L. Burnett
* Francois-Pier
* NikNaks
