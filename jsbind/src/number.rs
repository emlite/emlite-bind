use crate::bigint::BigInt;
use crate::error::{JsError, RangeError, SyntaxError, TypeError};
use crate::string::JsString;
use crate::utils::*;
use core::fmt;
use emlite::FromVal;

/// Wrapper around a JavaScript Number object
///
/// Number provides type-safe numeric conversions and validation operations,
/// corresponding to JavaScript's Number type. It handles NaN, infinity,
/// and range checking for safe conversions to native Rust numeric types.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Number {
    inner: emlite::Val,
}

bind!(Number);
impl_dyn_cast!(Number);

impl Number {
    /// Create a new Number from a double value
    pub fn new(value: f64) -> Self {
        Self {
            inner: value.into(),
        }
    }

    /// Create a new Number from a float value
    pub fn from_f32(value: f32) -> Self {
        Self::new(value as f64)
    }

    /// Create a new Number from a signed 32-bit integer
    pub fn from_i32(value: i32) -> Self {
        Self::new(value as f64)
    }

    /// Create a new Number from an unsigned 32-bit integer
    pub fn from_u32(value: u32) -> Self {
        Self::new(value as f64)
    }

    /// Create a new Number from a signed 64-bit integer
    pub fn from_i64(value: i64) -> Self {
        Self::new(value as f64)
    }

    /// Create a new Number from an unsigned 64-bit integer
    pub fn from_u64(value: u64) -> Self {
        Self::new(value as f64)
    }

    /// Create Number(0)
    pub fn zero() -> Self {
        Self::new(0.0)
    }

    /// Create Number(1)
    pub fn one() -> Self {
        Self::new(1.0)
    }

    // Validation methods
    /// Checks if value is NaN
    pub fn is_nan(&self) -> bool {
        emlite::Val::global("Number")
            .call("isNaN", &[self.inner.clone()])
            .as_::<bool>()
    }

    /// Checks if value is finite (not NaN, not ±infinity)
    pub fn is_finite(&self) -> bool {
        emlite::Val::global("Number")
            .call("isFinite", &[self.inner.clone()])
            .as_::<bool>()
    }

    /// Checks if value is an integer
    pub fn is_integer(&self) -> bool {
        emlite::Val::global("Number")
            .call("isInteger", &[self.inner.clone()])
            .as_::<bool>()
    }

    /// Checks if value is a safe integer (within Number.MAX_SAFE_INTEGER)
    pub fn is_safe_integer(&self) -> bool {
        emlite::Val::global("Number")
            .call("isSafeInteger", &[self.inner.clone()])
            .as_::<bool>()
    }

    // Safe conversion methods with error handling
    /// Converts to f64 with validation
    pub fn to_f64(&self) -> Result<f64, JsError> {
        if self.is_nan() {
            return Err(TypeError::new("Value is NaN").into());
        }
        if !self.is_finite() {
            return Err(TypeError::new("Value is not finite").into());
        }
        Ok(self.inner.as_::<f64>())
    }

    /// Converts to f32 with validation
    pub fn to_f32(&self) -> Result<f32, JsError> {
        let val = self.to_f64()?;
        if val > f32::MAX as f64 || val < f32::MIN as f64 {
            return Err(RangeError::new("Value out of f32 range").into());
        }
        Ok(val as f32)
    }

    /// Converts to signed 32-bit integer with validation
    pub fn to_i32(&self) -> Result<i32, JsError> {
        let val = self.to_f64()?;
        if val < i32::MIN as f64 || val > i32::MAX as f64 {
            return Err(RangeError::new("Value out of i32 range").into());
        }
        Ok(val as i32)
    }

    /// Converts to unsigned 32-bit integer with validation
    pub fn to_u32(&self) -> Result<u32, JsError> {
        let val = self.to_f64()?;
        if val < 0.0 || val > u32::MAX as f64 {
            return Err(RangeError::new("Value out of u32 range").into());
        }
        Ok(val as u32)
    }

    /// Converts to signed 64-bit integer with validation
    pub fn to_i64(&self) -> Result<i64, JsError> {
        let val = self.to_f64()?;
        if !self.fits_in_i64() {
            return Err(RangeError::new("Value out of i64 range or not safe integer").into());
        }
        Ok(val as i64)
    }

    /// Converts to unsigned 64-bit integer with validation
    pub fn to_u64(&self) -> Result<u64, JsError> {
        let val = self.to_f64()?;
        if !self.fits_in_u64() {
            return Err(RangeError::new("Value out of u64 range or not safe integer").into());
        }
        Ok(val as u64)
    }

    // Math operations
    /// Gets absolute value
    pub fn abs(&self) -> Self {
        Self {
            inner: emlite::Val::global("Math").call("abs", &[self.inner.clone()]),
        }
    }

    /// Rounds down to nearest integer
    pub fn floor(&self) -> Self {
        Self {
            inner: emlite::Val::global("Math").call("floor", &[self.inner.clone()]),
        }
    }

    /// Rounds up to nearest integer
    pub fn ceil(&self) -> Self {
        Self {
            inner: emlite::Val::global("Math").call("ceil", &[self.inner.clone()]),
        }
    }

    /// Rounds to nearest integer
    pub fn round(&self) -> Self {
        Self {
            inner: emlite::Val::global("Math").call("round", &[self.inner.clone()]),
        }
    }

    /// Truncates to integer (removes fractional part)
    pub fn trunc(&self) -> Self {
        Self {
            inner: emlite::Val::global("Math").call("trunc", &[self.inner.clone()]),
        }
    }

    // String conversion
    /// Converts Number to string with specified radix
    pub fn to_string_radix(&self, radix: i32) -> JsString {
        if radix == 10 {
            self.inner.call("toString", &[]).as_::<JsString>()
        } else {
            self.inner
                .call("toString", &[radix.into()])
                .as_::<JsString>()
        }
    }

    /// Gets primitive number value as string
    pub fn value_of(&self) -> JsString {
        self.inner.call("valueOf", &[]).as_::<JsString>()
    }

    /// Converts to locale-specific string
    pub fn to_locale_string(&self) -> JsString {
        self.inner.call("toLocaleString", &[]).as_::<JsString>()
    }

    /// Converts to exponential notation
    pub fn to_exponential(&self, fraction_digits: Option<i32>) -> JsString {
        match fraction_digits {
            Some(digits) => self
                .inner
                .call("toExponential", &[digits.into()])
                .as_::<JsString>(),
            None => self.inner.call("toExponential", &[]).as_::<JsString>(),
        }
    }

    /// Converts to fixed-point notation
    pub fn to_fixed(&self, digits: i32) -> JsString {
        self.inner
            .call("toFixed", &[digits.into()])
            .as_::<JsString>()
    }

    /// Converts to precision notation
    pub fn to_precision(&self, precision: Option<i32>) -> JsString {
        match precision {
            Some(prec) => self
                .inner
                .call("toPrecision", &[prec.into()])
                .as_::<JsString>(),
            None => self.inner.call("toPrecision", &[]).as_::<JsString>(),
        }
    }

    // BigInt conversion
    /// Converts to BigInt with validation
    pub fn to_bigint(&self) -> Result<BigInt, JsError> {
        if !self.is_integer() {
            return Err(TypeError::new("Cannot convert non-integer to BigInt").into());
        }
        if !self.is_finite() {
            return Err(TypeError::new("Cannot convert non-finite value to BigInt").into());
        }

        let bigint_ctor = emlite::Val::global("BigInt");
        let result = bigint_ctor.call("BigInt", &[self.inner.clone()]);
        Ok(BigInt::from_val(&result))
    }

    /// Creates Number from BigInt (may lose precision)
    pub fn from_bigint(bigint: &BigInt) -> Self {
        Self {
            inner: emlite::Val::global("Number").call("Number", &[bigint.clone().into()]),
        }
    }

    // Range checking for typed arrays
    /// Checks if value fits in signed 64-bit integer range
    pub fn fits_in_i64(&self) -> bool {
        if !self.is_integer() || !self.is_finite() {
            return false;
        }

        let val = self.inner.as_::<f64>();
        val >= i64::MIN as f64 && val <= i64::MAX as f64 && self.is_safe_integer()
    }

    /// Checks if value fits in unsigned 64-bit integer range
    pub fn fits_in_u64(&self) -> bool {
        if !self.is_integer() || !self.is_finite() {
            return false;
        }

        let val = self.inner.as_::<f64>();
        val >= 0.0 && val <= u64::MAX as f64 && self.is_safe_integer()
    }

    // Static utility methods
    /// Parses Number from string with error checking
    pub fn parse(s: &JsString) -> Result<Self, JsError> {
        let result = emlite::Val::global("Number").call("Number", &[s.clone().into()]);
        let num = Self::from_val(&result);
        if num.is_nan() {
            Err(SyntaxError::new("Failed to parse number from string").into())
        } else {
            Ok(num)
        }
    }

    /// Parses floating point number from string
    pub fn parse_float(s: &JsString) -> Result<Self, JsError> {
        let result = emlite::Val::global("parseFloat").call("parseFloat", &[s.clone().into()]);
        let num = Self::from_val(&result);
        if num.is_nan() {
            Err(SyntaxError::new("Failed to parse float from string").into())
        } else {
            Ok(num)
        }
    }

    /// Parses integer from string with radix
    pub fn parse_int(s: &JsString, radix: i32) -> Result<Self, JsError> {
        let result =
            emlite::Val::global("parseInt").call("parseInt", &[s.clone().into(), radix.into()]);
        let num = Self::from_val(&result);
        if num.is_nan() {
            Err(SyntaxError::new("Failed to parse integer from string").into())
        } else {
            Ok(num)
        }
    }

    // Constants
    /// Gets Number.EPSILON
    pub fn epsilon() -> Self {
        Self {
            inner: emlite::Val::global("Number").get("EPSILON"),
        }
    }

    /// Gets Number.MAX_VALUE
    pub fn max_value() -> Self {
        Self {
            inner: emlite::Val::global("Number").get("MAX_VALUE"),
        }
    }

    /// Gets Number.MIN_VALUE
    pub fn min_value() -> Self {
        Self {
            inner: emlite::Val::global("Number").get("MIN_VALUE"),
        }
    }

    /// Gets Number.MAX_SAFE_INTEGER
    pub fn max_safe_integer() -> Self {
        Self {
            inner: emlite::Val::global("Number").get("MAX_SAFE_INTEGER"),
        }
    }

    /// Gets Number.MIN_SAFE_INTEGER
    pub fn min_safe_integer() -> Self {
        Self {
            inner: emlite::Val::global("Number").get("MIN_SAFE_INTEGER"),
        }
    }

    /// Gets Number.POSITIVE_INFINITY
    pub fn positive_infinity() -> Self {
        Self {
            inner: emlite::Val::global("Number").get("POSITIVE_INFINITY"),
        }
    }

    /// Gets Number.NEGATIVE_INFINITY
    pub fn negative_infinity() -> Self {
        Self {
            inner: emlite::Val::global("Number").get("NEGATIVE_INFINITY"),
        }
    }

    /// Gets Number.NaN
    pub fn nan() -> Self {
        Self {
            inner: emlite::Val::global("Number").get("NaN"),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_string_radix(10);
        write!(f, "{}", s.to_string())
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::zero()
    }
}

// Conversion traits for common numeric types
impl From<f64> for Number {
    fn from(val: f64) -> Self {
        Self::new(val)
    }
}

impl From<f32> for Number {
    fn from(val: f32) -> Self {
        Self::from_f32(val)
    }
}

impl From<i32> for Number {
    fn from(val: i32) -> Self {
        Self::from_i32(val)
    }
}

impl From<u32> for Number {
    fn from(val: u32) -> Self {
        Self::from_u32(val)
    }
}

impl From<i64> for Number {
    fn from(val: i64) -> Self {
        Self::from_i64(val)
    }
}

impl From<u64> for Number {
    fn from(val: u64) -> Self {
        Self::from_u64(val)
    }
}
