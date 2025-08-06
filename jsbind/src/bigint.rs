use crate::error::{RangeError, SyntaxError};
use crate::utils::bind;
use alloc::format;
use alloc::string::String;

/// JavaScript BigInt type.
///
/// BigInt is a built-in object in JavaScript that provides a way to represent
/// whole numbers larger than 2^53 - 1.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct BigInt {
    inner: emlite::Val,
}

bind!(BigInt);

impl BigInt {
    /// Creates a new BigInt from a string.
    ///
    /// # Arguments
    /// * `value` - String representation of the number
    ///
    /// # Returns
    /// Result containing the BigInt or a SyntaxError if the string is invalid
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let big_num = BigInt::from_str("123456789012345678901234567890").unwrap();
    /// assert!(BigInt::from_str("invalid").is_err());
    /// ```
    pub fn from_str(value: &str) -> Result<Self, SyntaxError> {
        let ctor = emlite::Val::global("BigInt");
        let result = ctor.invoke(&[value.into()]);
        result.as_::<Result<Self, SyntaxError>>()
    }

    /// Creates a new BigInt from an i64.
    ///
    /// # Arguments  
    /// * `value` - The integer value
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let big_num = BigInt::from_i64(9223372036854775807i64);
    /// ```
    pub fn from_i64(value: i64) -> Self {
        let ctor = emlite::Val::global("BigInt");
        let val = ctor.invoke(&[value.into()]);
        Self { inner: val }
    }

    /// Creates a new BigInt from a u64.
    ///
    /// # Arguments
    /// * `value` - The unsigned integer value  
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let big_num = BigInt::from_u64(18446744073709551615u64);
    /// ```
    pub fn from_u64(value: u64) -> Self {
        let ctor = emlite::Val::global("BigInt");
        let val = ctor.invoke(&[value.into()]);
        Self { inner: val }
    }

    /// Converts this BigInt to a string representation.
    ///
    /// # Arguments
    /// * `radix` - Optional radix (base) for the conversion (2-36)
    ///
    /// # Returns
    /// String representation of this BigInt
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let big_num = BigInt::from_str("255");
    /// assert_eq!(big_num.to_string_with_radix(Some(16)), "ff");
    /// assert_eq!(big_num.to_string_with_radix(None), "255");
    /// ```
    pub fn to_string_with_radix(&self, radix: Option<u32>) -> String {
        match radix {
            Some(r) => self
                .inner
                .call("toString", &[r.into()])
                .as_::<Option<String>>()
                .unwrap_or_default(),
            None => self
                .inner
                .call("toString", &[])
                .as_::<Option<String>>()
                .unwrap_or_default(),
        }
    }

    /// Converts this BigInt to a string representation (base 10).
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let big_num = BigInt::from_str("123456789");
    /// assert_eq!(big_num.to_string_repr(), "123456789");
    /// ```
    pub fn to_string_repr(&self) -> String {
        self.to_string_with_radix(None)
    }

    /// Returns the primitive value of this BigInt.
    ///
    /// # Examples
    /// ```rust  
    /// use jsbind::prelude::*;
    ///
    /// let big_num = BigInt::from_str("42");
    /// let primitive = big_num.value_of();
    /// ```
    pub fn value_of(&self) -> Self {
        let val = self.inner.call("valueOf", &[]);
        Self { inner: val }
    }

    /// Converts this BigInt to a standard Rust i64.
    ///
    /// # Returns
    /// `Result<i64, RangeError>` with specific error details if conversion fails
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let small = BigInt::from_i64(42);
    /// assert!(small.to_i64().is_ok());
    ///
    /// let too_big = BigInt::from_str("999999999999999999999999999999").unwrap();
    /// assert!(too_big.to_i64().is_err());
    /// ```
    pub fn to_i64(&self) -> Result<i64, RangeError> {
        // Check if it's within i64 range by comparing with MAX/MIN
        let max_check = emlite::Val::global("eval").invoke(&[format!(
            "({}) <= {}n",
            self.to_string_repr(),
            i64::MAX
        )
        .into()]);
        let min_check = emlite::Val::global("eval").invoke(&[format!(
            "({}) >= {}n",
            self.to_string_repr(),
            i64::MIN
        )
        .into()]);

        if max_check.as_::<bool>() && min_check.as_::<bool>() {
            Ok(emlite::Val::global("Number")
                .invoke(&[self.inner.clone()])
                .as_::<i64>())
        } else {
            Err(RangeError::new(&format!(
                "BigInt value {} is out of i64 range",
                self.to_string_repr()
            )))
        }
    }

    /// Converts this BigInt to a standard Rust u64.
    ///
    /// # Returns
    /// `Result<u64, RangeError>` with specific error details if conversion fails
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let positive = BigInt::from_u64(42);
    /// assert!(positive.to_u64().is_ok());
    ///
    /// let negative = BigInt::from_i64(-1);
    /// assert!(negative.to_u64().is_err());
    /// ```
    pub fn to_u64(&self) -> Result<u64, RangeError> {
        // Check if non-negative and within u64 range
        let pos_check =
            emlite::Val::global("eval")
                .invoke(&[format!("({}) >= 0n", self.to_string_repr()).into()]);
        let max_check = emlite::Val::global("eval").invoke(&[format!(
            "({}) <= {}n",
            self.to_string_repr(),
            u64::MAX
        )
        .into()]);

        if pos_check.as_::<bool>() && max_check.as_::<bool>() {
            Ok(emlite::Val::global("Number")
                .invoke(&[self.inner.clone()])
                .as_::<u64>())
        } else if !pos_check.as_::<bool>() {
            Err(RangeError::new(&format!(
                "BigInt value {} is negative and cannot be converted to u64",
                self.to_string_repr()
            )))
        } else {
            Err(RangeError::new(&format!(
                "BigInt value {} is too large for u64",
                self.to_string_repr()
            )))
        }
    }
}

impl crate::prelude::DynCast for BigInt {
    fn instanceof(val: &emlite::Val) -> bool {
        val.type_of() == "bigint"
    }

    fn unchecked_from_val(v: emlite::Val) -> Self {
        Self { inner: v }
    }

    fn unchecked_from_val_ref(v: &emlite::Val) -> &Self {
        unsafe { &*(v as *const emlite::Val as *const Self) }
    }

    fn unchecked_from_val_mut(v: &mut emlite::Val) -> &mut Self {
        unsafe { &mut *(v as *mut emlite::Val as *mut Self) }
    }
}

impl core::fmt::Display for BigInt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}
