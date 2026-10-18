# Native decimal integration

This branch targets vivainio/luau's native decimal VM. Applications can use it
with one dependency; Cargo fetches the matching Luau sources automatically:

```toml
[dependencies]
mlua = { git = "https://github.com/vivainio/mlua-decimal-native", branch = "main", features = ["luau"] }
```

Commit your application's `Cargo.lock` and run `cargo build --locked` in CI.
For an immutable dependency declaration, replace `branch` with `rev` containing
a tested commit from this branch. No sibling checkouts or system Luau are needed.
A Rust toolchain and a C++ compiler supporting `__int128` (GCC or Clang on
Linux/macOS) are required. Windows/MSVC is currently unsupported.

The `mlua-sys` crate pins `luau-decimal-src-rs` to an exact Git revision so the
compiler, VM, and bindings use compatible versions. If another dependency uses
crates.io mlua, apply the same Git source to `mlua` and `mlua-sys` under your
workspace's `[patch.crates-io]` section so the dependency graph uses this fork.

```rust
use mlua::{Decimal, Lua, Result};

fn main() -> Result<()> {
    let lua = Lua::new();
    lua.globals().set("price", Decimal::new(1999, 2).unwrap())?;
    let total: Decimal = lua.load("return price * integer.create(3)").eval()?;
    assert_eq!(total.to_string(), "59.97");
    Ok(())
}
```

Enable `mlua`'s `luau` feature. `Lua::new()` opens the decimal library;
selective initialization can request `StdLib::DECIMAL`.
`Decimal::new(mantissa, scale)` validates the signed 90-bit magnitude and
scale 0..=28. `Value::Decimal`, `IntoLua`, and `FromLua` preserve the exact
coefficient and scale. Equality compares numeric values across scales.
There are no implicit Rust decimal/float or decimal/integer conversions.
With `serde`, native decimal values serialize and deserialize as exact decimal
strings; deserializing a string does not automatically create a native decimal.

This replaces Luau's ABI, including its type tags. Do not link these bindings
against stock Luau. The vendored VM and compiler must be updated together.
The fork currently requires a C++ compiler supporting `__int128`.

Validation: `cargo test --features luau,serde --test decimal`.
