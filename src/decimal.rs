use std::{cmp::Ordering, fmt};

/// A native Luau decimal with a signed 90-bit magnitude and scale from 0 to 28.
/// Construction and conversion preserve precision; arithmetic is performed by Luau.
#[derive(Clone, Copy, Debug)]
pub struct Decimal {
    mantissa: i128,
    scale: u32,
}

impl Decimal {
    /// Constructs `mantissa * 10^(-scale)`, returning `None` outside Luau's range.
    pub const fn new(mantissa: i128, scale: u32) -> Option<Self> {
        if mantissa.unsigned_abs() >= (1u128 << 90) || scale > 28 {
            return None;
        }
        Some(Self { mantissa, scale })
    }

    /// Returns the signed coefficient, without removing trailing zeros.
    pub const fn mantissa(self) -> i128 {
        self.mantissa
    }
    /// Returns the number of digits after the decimal point.
    pub const fn scale(self) -> u32 {
        self.scale
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digits = self.mantissa.unsigned_abs().to_string();
        if self.mantissa < 0 {
            f.write_str("-")?;
        }
        let scale = self.scale as usize;
        if scale == 0 {
            return f.write_str(&digits);
        }
        if digits.len() <= scale {
            write!(f, "0.{}{}", "0".repeat(scale - digits.len()), digits)
        } else {
            let split = digits.len() - scale;
            write!(f, "{}.{}", &digits[..split], &digits[split..])
        }
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let sign = self.mantissa.signum().cmp(&other.mantissa.signum());
        if sign != Ordering::Equal {
            return sign;
        }
        let scale = self.scale.max(other.scale);
        let a = format!(
            "{}{}",
            self.mantissa.unsigned_abs(),
            "0".repeat((scale - self.scale) as usize)
        );
        let b = format!(
            "{}{}",
            other.mantissa.unsigned_abs(),
            "0".repeat((scale - other.scale) as usize)
        );
        if self.mantissa == 0 {
            return Ordering::Equal;
        }
        let order = a.len().cmp(&b.len()).then_with(|| a.cmp(&b));
        if self.mantissa < 0 { order.reverse() } else { order }
    }
}
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Decimal {}

impl crate::IntoLua for Decimal {
    fn into_lua(self, _: &crate::Lua) -> crate::Result<crate::Value> {
        Ok(crate::Value::Decimal(self))
    }
}
impl crate::FromLua for Decimal {
    fn from_lua(value: crate::Value, _: &crate::Lua) -> crate::Result<Self> {
        match value {
            crate::Value::Decimal(d) => Ok(d),
            _ => Err(crate::Error::from_lua_conversion(
                value.type_name(),
                "decimal",
                None,
            )),
        }
    }
}
