use mlua::{Decimal, Lua, Result};

fn main() -> Result<()> {
    let lua = Lua::new();
    lua.globals().set("price", Decimal::new(1999, 2).unwrap())?;
    let total: Decimal = lua.load("return price * 3d").eval()?;
    assert_eq!(total.to_string(), "59.97");
    println!("{total}");
    Ok(())
}
