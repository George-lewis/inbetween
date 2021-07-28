# inbetween

A small macro that makes checking bounds look slightly nicer

### Example

```rust
use inbetween::between;

let x = 33;

if between!(20 < x < 30) {
  println!("This prints!");
}

if between![0 < x > 10] {
  println!("This also prints!");
}

if between! { 99 > x >= 33 } {
  println!("...");
}
```
