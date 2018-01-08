# id

Make unique identifiers.

```toml
[dependencies]
id = "0.1"
```

## Usage

```rust
#[macro_use]
extern crate id;

...

let x = id!();
```

Each ID identifies a unique point in the source code, so you can't make more of them at runtime with a loop, nor with other such shenanigans.

## License

Just take the code and do whatever you want with it, which I assume isn't much, because this library is pretty much useless.
