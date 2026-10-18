# mlua-decimal-native

[![Decimal CI](https://github.com/vivainio/mlua-decimal-native/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/vivainio/mlua-decimal-native/actions/workflows/main.yml)

Experimental Rust bindings for **native decimal values in Luau**. This is a fork
of [mlua](https://github.com/mlua-rs/mlua) that embeds
[vivainio/luau](https://github.com/vivainio/luau)'s decimal implementation.

Use it when your Rust application embeds Luau and you want scripts to calculate
with exact base-10 values, such as prices and quantities. Decimals are a primitive
VM value stored inline, with literals such as `19.99d` and arithmetic operators.
Rust can pass them into scripts and receive them back without converting through
floating point or allocating a userdata wrapper.

This repository's `main` is the application-facing decimal fork and is maintained
separately from the branch intended for upstream contribution. It is experimental;
the API and the underlying Luau implementation may change. It is separate from
the [mlua-decimal](https://github.com/vivainio/mlua-decimal) userdata library.

## Use it in an application

Add this to your application's `Cargo.toml`:

```toml
[dependencies]
mlua = { git = "https://github.com/vivainio/mlua-decimal-native", branch = "main", features = ["luau"] }
```

The crate is still named `mlua`, so Rust imports remain `use mlua::...`.
Cargo fetches and builds the matching decimal Luau compiler and VM automatically
from a pinned revision of
[luau-decimal-src-rs](https://github.com/vivainio/luau-decimal-src-rs).
You do not need to clone that repository or install Luau separately.

Put this in `src/main.rs` and run `cargo run`:

```rust
use mlua::{Decimal, Lua, Result};

fn main() -> Result<()> {
    let lua = Lua::new();

    // 1999 × 10^-2 = 19.99, represented exactly.
    lua.globals().set("price", Decimal::new(1999, 2).unwrap())?;

    let total: Decimal = lua.load("return price * 3d").eval()?;
    assert_eq!(total.to_string(), "59.97");
    println!("{total}");
    Ok(())
}
```

`Decimal::new(mantissa, scale)` returns `None` if the magnitude exceeds 90 bits
or the scale exceeds 28. `mantissa()` and `scale()` let Rust inspect the exact
stored value. Native decimals also appear as `Value::Decimal` when working
with dynamically typed values, and work in typed Rust callbacks through
`IntoLua` and `FromLua`.

## Use decimals in scripts

`Lua::new()` opens the `decimal` library automatically:

```lua
local price = decimal.fromstring("19.99")
assert(price * 3d == 59.97d)
assert(0.1d + 0.2d == 0.3d)
assert(decimal.round(1.255d, 2) == 1.26d)
print(tostring(price)) -- 19.99
```

Use the `d` suffix for decimal literals. Plain literals such as `3` are Luau
numbers; decimals do not mix implicitly with those floating point values.
When choosing libraries explicitly with `Lua::new_with`, include
`StdLib::DECIMAL` to load the decimal library.

With the `serde` feature, native decimal values serialize as decimal strings to
preserve precision. See [DECIMAL.md](DECIMAL.md) for conversion behavior and
binding details.

## Builds and CI

The decimal backend is tested on Linux and macOS with GCC or Clang. It needs a
C++ compiler supporting `__int128`; Windows/MSVC is currently unsupported.
Enable `luau` as shown above; the other Lua backends do not provide this native
decimal type.

Commit your application's `Cargo.lock`, then build in CI with:

```sh
cargo build --locked
```

For an immutable dependency declaration, replace `branch = "main"` with
`rev = "<tested commit SHA>"`. The CI in this repository tests the bindings,
JIT/vector4 configurations, and a standalone application that fetches the fork
through its Git dependency.

To try the example or tests from a checkout of this repository:

```sh
cargo run --features luau --example decimal
cargo test --features luau,serde --test decimal
```

The bindings and decimal VM have a shared ABI. Use the pinned source dependency
included here when updating an application; stock Luau is not interchangeable
with this fork.

## More documentation

- [Decimal integration details](DECIMAL.md)
- [Runnable Rust example](examples/decimal.rs)
- [Decimal integration tests](tests/decimal.rs)
- [Upstream mlua README](UPSTREAM_README.md) for general features and examples;
  its release badges and platform claims describe upstream mlua.

This project retains mlua's [MIT license](LICENSE).
