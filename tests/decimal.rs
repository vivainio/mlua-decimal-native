#![cfg(feature = "luau")]

use mlua::{Decimal, Function, Lua, LuaOptions, Result, StdLib, Value};

#[test]
fn decimal_round_trip() -> Result<()> {
    let lua = Lua::new();
    let identity: Function = lua.load("return function(x) return x end").eval()?;
    for (mantissa, scale) in [
        (1999, 2),
        (-12300, 4),
        (0, 28),
        ((1i128 << 90) - 1, 0),
        (-((1i128 << 90) - 1), 28),
        (1, 28),
    ] {
        let value = Decimal::new(mantissa, scale).unwrap();
        let result: Decimal = identity.call(value)?;
        assert_eq!(result.mantissa(), mantissa);
        assert_eq!(result.scale(), scale);
        lua.globals().set("d", value)?;
        assert_eq!(lua.load("tostring(d)").eval::<String>()?, value.to_string());
        let dynamic: Value = identity.call(value)?;
        assert!(dynamic.is_decimal());
        assert_eq!(dynamic.as_decimal(), Some(value));
        assert_eq!(dynamic.type_name(), "decimal");
    }
    Ok(())
}

#[test]
fn decimal_scripts_and_callbacks() -> Result<()> {
    let lua = Lua::new();
    lua.globals()
        .set("echo", lua.create_function(|_, d: Decimal| Ok(d))?)?;
    lua.load(
        r#"
        assert(type(19.99d) == "decimal")
        assert(echo(19.99d) == 19.99d)
        assert(19.99d * integer.create(3) == 59.97d)
        assert(decimal.fromstring("0.10") + 0.20d == 0.30d)
        assert(tostring(decimal.round(1.255d, 2)) == "1.26")
    "#,
    )
    .exec()?;
    let result: Decimal = lua.load("return 19.99d * integer.create(3)").eval()?;
    assert_eq!(result, Decimal::new(5997, 2).unwrap());
    assert!(lua.load("return 1.5").eval::<Decimal>().is_err());
    assert!(lua.load("return 1d").eval::<f64>().is_err());
    assert!(lua.load("return 1d + 1").eval::<Decimal>().is_err());
    Ok(())
}

#[test]
fn decimal_library_selection_and_type_tags() -> Result<()> {
    let lua = Lua::new_with(StdLib::DECIMAL, LuaOptions::default())?;
    assert_eq!(
        lua.load("return decimal.fromstring('2.50')").eval::<Decimal>()?,
        Decimal::new(250, 2).unwrap()
    );
    let lua = Lua::new_with(StdLib::NONE, LuaOptions::default())?;
    assert!(lua.globals().get::<Value>("decimal")?.is_nil());
    let lua = Lua::new();
    lua.load(
        r#"
        assert(type("s") == "string")
        assert(type({}) == "table")
        assert(type(function() end) == "function")
        assert(type(coroutine.create(function() end)) == "thread")
        assert(type(buffer.create(1)) == "buffer")
        assert(type(vector.create(1, 2, 3)) == "vector")
    "#,
    )
    .exec()?;
    Ok(())
}

#[test]
fn decimal_validation_and_comparison() {
    assert!(Decimal::new(1i128 << 90, 0).is_none());
    assert!(Decimal::new(-(1i128 << 90), 0).is_none());
    assert!(Decimal::new(i128::MIN, 0).is_none());
    assert!(Decimal::new(0, 29).is_none());
    let d = |m, s| Decimal::new(m, s).unwrap();
    assert_eq!(d(100, 2), d(1, 0));
    assert_eq!(d(0, 28), d(0, 0));
    assert!(d(-101, 2) < d(-1, 0));
    assert!(d(1, 28) < d(1, 0));
    assert!(d(999, 2) < d(10, 0));
    assert_eq!(d(-1, 28).to_string(), "-0.0000000000000000000000000001");
}

#[cfg(feature = "serde")]
#[test]
fn decimal_serde_preserves_precision() -> Result<()> {
    use mlua::LuaSerdeExt;
    let lua = Lua::new();
    let value: Value = lua.load("return 12345678901234567890123456.78d").eval()?;
    let expected = "12345678901234567890123456.78";
    assert_eq!(serde_json::to_string(&value).unwrap(), format!("\"{expected}\""));
    assert_eq!(lua.from_value::<String>(value)?, expected);
    Ok(())
}
