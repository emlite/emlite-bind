use crate::error::{RangeError, SyntaxError};
use crate::utils::bind;
use alloc::format;
use alloc::str::FromStr;
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

impl FromStr for BigInt {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ctor = emlite::Val::global("BigInt");
        let result = ctor.invoke(&[s.into()]);
        result.as_::<Result<Self, SyntaxError>>()
    }
}

impl BigInt {
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

    /// Power operation
    pub fn pow(&self, other: &BigInt) -> BigInt {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) ** ({})",
            self.to_string_repr(),
            other.to_string_repr()
        )
        .into()]);
        Self { inner: result }
    }

    /// Gets hash code for BigInt  
    pub fn get_hash(&self) -> u64 {
        use emlite::FromVal;
        // Simple hash based on the handle value
        self.inner.as_handle() as u64
    }

    /// Checks if BigInt equals zero
    pub fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    /// Checks if BigInt is positive
    pub fn is_positive(&self) -> bool {
        self > &Self::zero()
    }

    /// Checks if BigInt is negative
    pub fn is_negative(&self) -> bool {
        self < &Self::zero()
    }

    /// Gets absolute value
    pub fn abs(&self) -> BigInt {
        if self.is_negative() {
            -self
        } else {
            self.clone()
        }
    }

    /// Creates BigInt(0)
    pub fn zero() -> BigInt {
        Self::from_i64(0)
    }

    /// Creates BigInt(1)
    pub fn one() -> BigInt {
        Self::from_i64(1)
    }

    /// Creates BigInt(-1)
    pub fn minus_one() -> BigInt {
        Self::from_i64(-1)
    }

    /// Returns signed n-bit BigInt value
    pub fn as_int_n(width: u32, bigint: &BigInt) -> BigInt {
        let result =
            emlite::Val::global("BigInt").call("asIntN", &[width.into(), bigint.inner.clone()]);
        Self { inner: result }
    }

    /// Returns unsigned n-bit BigInt value  
    pub fn as_uint_n(width: u32, bigint: &BigInt) -> BigInt {
        let result =
            emlite::Val::global("BigInt").call("asUintN", &[width.into(), bigint.inner.clone()]);
        Self { inner: result }
    }

    /// Converts BigInt to locale-specific string
    pub fn to_locale_string(&self) -> String {
        self.inner
            .call("toLocaleString", &[])
            .as_::<Option<String>>()
            .unwrap_or_default()
    }

    /// Parses BigInt from string with error checking
    pub fn parse(s: &str) -> Result<BigInt, SyntaxError> {
        Self::from_str(s)
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

// Arithmetic operators
impl core::ops::Add for BigInt {
    type Output = BigInt;
    fn add(self, rhs: BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) + ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Add for &BigInt {
    type Output = BigInt;
    fn add(self, rhs: &BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) + ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Sub for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) - ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Sub for &BigInt {
    type Output = BigInt;
    fn sub(self, rhs: &BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) - ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Mul for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) * ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Mul for &BigInt {
    type Output = BigInt;
    fn mul(self, rhs: &BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) * ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Div for BigInt {
    type Output = BigInt;
    fn div(self, rhs: BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) / ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Div for &BigInt {
    type Output = BigInt;
    fn div(self, rhs: &BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) / ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Rem for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) % ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Rem for &BigInt {
    type Output = BigInt;
    fn rem(self, rhs: &BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) % ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Neg for BigInt {
    type Output = BigInt;
    fn neg(self) -> Self::Output {
        let result =
            emlite::Val::global("eval").invoke(&[format!("-({})", self.to_string_repr()).into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Neg for &BigInt {
    type Output = BigInt;
    fn neg(self) -> Self::Output {
        let result =
            emlite::Val::global("eval").invoke(&[format!("-({})", self.to_string_repr()).into()]);
        BigInt { inner: result }
    }
}

// Bitwise operators
impl core::ops::BitAnd for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) & ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::BitAnd for &BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: &BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) & ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::BitOr for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) | ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::BitOr for &BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: &BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) | ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::BitXor for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) ^ ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::BitXor for &BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: &BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) ^ ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Not for BigInt {
    type Output = BigInt;
    fn not(self) -> Self::Output {
        let result =
            emlite::Val::global("eval").invoke(&[format!("~({})", self.to_string_repr()).into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Not for &BigInt {
    type Output = BigInt;
    fn not(self) -> Self::Output {
        let result =
            emlite::Val::global("eval").invoke(&[format!("~({})", self.to_string_repr()).into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Shl<BigInt> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) << ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Shl<&BigInt> for &BigInt {
    type Output = BigInt;
    fn shl(self, rhs: &BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) << ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Shr<BigInt> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) >> ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

impl core::ops::Shr<&BigInt> for &BigInt {
    type Output = BigInt;
    fn shr(self, rhs: &BigInt) -> Self::Output {
        let result = emlite::Val::global("eval").invoke(&[format!(
            "({}) >> ({})",
            self.to_string_repr(),
            rhs.to_string_repr()
        )
        .into()]);
        BigInt { inner: result }
    }
}

// Comparison operators
impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        emlite::Val::global("eval")
            .invoke(&[format!(
                "({}) === ({})",
                self.to_string_repr(),
                other.to_string_repr()
            )
            .into()])
            .as_::<bool>()
    }
}

impl Eq for BigInt {}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let lt = emlite::Val::global("eval")
            .invoke(&[format!("({}) < ({})", self.to_string_repr(), other.to_string_repr()).into()])
            .as_::<bool>();
        let gt = emlite::Val::global("eval")
            .invoke(&[format!("({}) > ({})", self.to_string_repr(), other.to_string_repr()).into()])
            .as_::<bool>();

        if lt {
            core::cmp::Ordering::Less
        } else if gt {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

impl core::hash::Hash for BigInt {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        use emlite::FromVal;
        self.inner.as_handle().hash(state);
    }
}

// Assignment operators
impl core::ops::AddAssign for BigInt {
    fn add_assign(&mut self, rhs: BigInt) {
        *self = core::mem::take(self) + rhs;
    }
}

impl core::ops::AddAssign<&BigInt> for BigInt {
    fn add_assign(&mut self, rhs: &BigInt) {
        *self = &*self + rhs;
    }
}

impl core::ops::SubAssign for BigInt {
    fn sub_assign(&mut self, rhs: BigInt) {
        *self = core::mem::take(self) - rhs;
    }
}

impl core::ops::SubAssign<&BigInt> for BigInt {
    fn sub_assign(&mut self, rhs: &BigInt) {
        *self = &*self - rhs;
    }
}

impl core::ops::MulAssign for BigInt {
    fn mul_assign(&mut self, rhs: BigInt) {
        *self = core::mem::take(self) * rhs;
    }
}

impl core::ops::MulAssign<&BigInt> for BigInt {
    fn mul_assign(&mut self, rhs: &BigInt) {
        *self = &*self * rhs;
    }
}

impl core::ops::DivAssign for BigInt {
    fn div_assign(&mut self, rhs: BigInt) {
        *self = core::mem::take(self) / rhs;
    }
}

impl core::ops::DivAssign<&BigInt> for BigInt {
    fn div_assign(&mut self, rhs: &BigInt) {
        *self = &*self / rhs;
    }
}

impl core::ops::RemAssign for BigInt {
    fn rem_assign(&mut self, rhs: BigInt) {
        *self = core::mem::take(self) % rhs;
    }
}

impl core::ops::RemAssign<&BigInt> for BigInt {
    fn rem_assign(&mut self, rhs: &BigInt) {
        *self = &*self % rhs;
    }
}

impl core::ops::BitAndAssign for BigInt {
    fn bitand_assign(&mut self, rhs: BigInt) {
        *self = core::mem::take(self) & rhs;
    }
}

impl core::ops::BitAndAssign<&BigInt> for BigInt {
    fn bitand_assign(&mut self, rhs: &BigInt) {
        *self = &*self & rhs;
    }
}

impl core::ops::BitOrAssign for BigInt {
    fn bitor_assign(&mut self, rhs: BigInt) {
        *self = core::mem::take(self) | rhs;
    }
}

impl core::ops::BitOrAssign<&BigInt> for BigInt {
    fn bitor_assign(&mut self, rhs: &BigInt) {
        *self = &*self | rhs;
    }
}

impl core::ops::BitXorAssign for BigInt {
    fn bitxor_assign(&mut self, rhs: BigInt) {
        *self = core::mem::take(self) ^ rhs;
    }
}

impl core::ops::BitXorAssign<&BigInt> for BigInt {
    fn bitxor_assign(&mut self, rhs: &BigInt) {
        *self = &*self ^ rhs;
    }
}

impl core::ops::ShlAssign<BigInt> for BigInt {
    fn shl_assign(&mut self, rhs: BigInt) {
        *self = core::mem::take(self) << rhs;
    }
}

impl core::ops::ShlAssign<&BigInt> for BigInt {
    fn shl_assign(&mut self, rhs: &BigInt) {
        *self = &*self << rhs;
    }
}

impl core::ops::ShrAssign<BigInt> for BigInt {
    fn shr_assign(&mut self, rhs: BigInt) {
        *self = core::mem::take(self) >> rhs;
    }
}

impl core::ops::ShrAssign<&BigInt> for BigInt {
    fn shr_assign(&mut self, rhs: &BigInt) {
        *self = &*self >> rhs;
    }
}

impl Default for BigInt {
    fn default() -> Self {
        Self::zero()
    }
}
