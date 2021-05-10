# Gridit

A 2D grid library utilizing the fun of iterators.


`Gridit` is a experimental 2D grid library which implements different Iterators 
to go over the cells of the grid. 
It was created with board games like chess in mind but can also be used for other use cases.

`Gridit` started to see how it would feel to create 
Chess movement patterns with iterators. The example chess board can be found
in `examples/board`.


Simple example: possible moves of the king without checking for check.
```rust
fn possible_moves(&self, grid: &Grid<BoardPiece>, current_pos: Position) -> Vec<Position> {
	grid.neighbors(pos)
		.grid_positions()
		.filter(|(pos, cell)| !matches!(cell, NonEmpty(piece) if piece.color == self.color))
		.map(|(pos, _)| pos)
		.collect()
}
```

## Examples

```
git clone https://github.com/jomsch/gridit.git
cd gridit
```

Run the examples as following:  
### Game Of Life
```
cargo run --example gameoflife
```

### Board
```
cargo run --example board
```
The piece with the `T` in it is a `Blocker` it can not beat any pieces and moves by teleporting
to a friendly piece.  
The `Giraffe` is a blank piece for testing purposes. If you want to quickly test this library
i recommend to clone this repository and implement some movement pattern for the [Giraffe](./examples/board/piece/giraffe.rs) piece.
Just write the `fn possible_moves` function and run the board example.
For inspiration see the chess piece implementation [here](./examples/board/piece).

## What's missing
These functions are missing.
Waiting for [generic associated types](https://github.com/rust-lang/rust/issues/44265)to land in rust.
- [ ] `pattern_mut`
- [ ] `neighbor_mut`

## Feedback & Questions
If you have any feedback or question,  
open an issue or shot me a message on Twitter.  
I would love to hear some thoughts and ideas on `Gridit`.




## Asset License
Assets found in [resources](./resources/) are licensed under [CC-BY-SA 3.0](https://creativecommons.org/licenses/by-sa/3.0/legalcode).
These assets can be found [here](https://commons.wikimedia.org/wiki/Category:SVG_chess_pieces).
Thanks and credit to:
* Colin M.L. Burnett
* Francois-Pier
* NikNaks

## License
Distributed under the MIT License. See [LICENSE.txt](./LICENSE.txt) for more information.
