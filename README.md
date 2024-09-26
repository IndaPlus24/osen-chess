# osen-chess
A chess engine

# Rules

- [x] Basic moves
- [x] Promotion
- [ ] Check

# Docs

Create docs and open in browser

```rust
cargo doc --open
```


## Example

```rust
fn main() {
    let mut game = Game::default();

    let pawn_move = game.make_move((Rank::A, File::Two), (Rank::A, File::Four));
    match pawn_move {
        Ok(_) => {println!("{game}")},
        Err(e) => {eprintln!("{e:?}")},
    }
}
```
