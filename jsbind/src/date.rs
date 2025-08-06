use crate::any::Any;
use crate::error::JsError;
use crate::string::JsString;
use crate::utils::*;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};

/// One-to-one wrapper around a JS `Date` instance.
#[derive(Clone, Debug, PartialOrd)]
#[repr(transparent)]
pub struct Date {
    inner: emlite::Val,
}

bind!(Date);
impl_dyn_cast!(Date);

impl Date {
    /// `new Date()` → current time.
    pub fn new_now() -> Self {
        let ctor = emlite::Val::global("Date");
        ctor.new(&[]).as_::<Self>()
    }

    /// `new Date(ms)` where ms is milliseconds since Unix epoch.
    pub fn from_epoch_millis(ms: i64) -> Self {
        let ctor = emlite::Val::global("Date");
        ctor.new(&[ms.into()]).as_::<Self>()
    }

    /// JS `Date.now()` (static form) → epoch milliseconds.
    pub fn now_epoch_millis() -> i64 {
        emlite::Val::global("Date").call("now", &[]).as_::<i64>()
    }

    /// Parses a date string and returns a new Date object.
    ///
    /// # Arguments
    /// * `date_string` - String representation of a date
    ///
    /// # Returns
    /// Result containing the Date object or a JsError if parsing fails
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let date = Date::parse("2023-12-25T00:00:00Z").unwrap();
    /// let invalid = Date::parse("not a date");
    /// assert!(invalid.is_err());
    /// ```
    pub fn parse(date_string: &str) -> Result<Self, JsError> {
        let timestamp = emlite::Val::global("Date").call("parse", &[date_string.into()]);

        // Date.parse returns NaN for invalid dates
        if crate::prelude::is_nan(&timestamp) {
            Err(JsError::new(&format!(
                "Invalid date string: '{}'",
                date_string
            )))
        } else {
            Ok(Self::from_epoch_millis(timestamp.as_::<i64>()))
        }
    }

    /// Creates a new Date from year, month, and optional day, hour, minute, second, millisecond.
    ///
    /// # Arguments
    /// * `year` - Full year (e.g., 2023)
    /// * `month` - Month (0-11, where 0 = January)
    /// * `day` - Optional day of month (1-31), defaults to 1
    /// * `hour` - Optional hour (0-23), defaults to 0
    /// * `minute` - Optional minute (0-59), defaults to 0
    /// * `second` - Optional second (0-59), defaults to 0
    /// * `millisecond` - Optional millisecond (0-999), defaults to 0
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let date = Date::from_components(2023, 11, Some(25), None, None, None, None); // December 25, 2023
    /// ```
    pub fn from_components(
        year: i32,
        month: i32,
        day: Option<i32>,
        hour: Option<i32>,
        minute: Option<i32>,
        second: Option<i32>,
        millisecond: Option<i32>,
    ) -> Self {
        let ctor = emlite::Val::global("Date");
        let mut args = vec![year.into(), month.into()];

        if let Some(d) = day {
            args.push(d.into());
        }
        if let Some(h) = hour {
            args.push(h.into());
        }
        if let Some(m) = minute {
            args.push(m.into());
        }
        if let Some(s) = second {
            args.push(s.into());
        }
        if let Some(ms) = millisecond {
            args.push(ms.into());
        }

        ctor.new(&args).as_::<Self>()
    }
}

impl Date {
    /// `Date.prototype.valueOf`
    /// (epoch milliseconds).
    #[inline]
    pub fn value_of(&self) -> i64 {
        self.inner.call("valueOf", &[]).as_::<i64>()
    }

    /// `Date.prototype.getTime`
    #[inline]
    pub fn get_time(&self) -> i64 {
        self.value_of()
    }

    /// `Date.prototype.toISOString`
    ///
    /// # Returns
    /// Result containing the ISO string or a RangeError for invalid dates
    ///
    /// # Examples
    /// ```rust
    /// use jsbind::prelude::*;
    ///
    /// let date = Date::from_epoch_millis(1640995200000);
    /// assert!(date.to_iso_string().is_ok());
    ///
    /// let invalid = Date::from_epoch_millis(8.64e15 as i64 + 1);
    /// assert!(invalid.to_iso_string().is_err());
    /// ```
    pub fn to_iso_string(&self) -> Result<JsString, JsError> {
        self.inner
            .call("toISOString", &[])
            .as_::<Result<JsString, JsError>>()
    }

    /// `Date.prototype.toUTCString`
    pub fn to_utc_string(&self) -> Option<String> {
        self.inner.call("toUTCString", &[]).as_::<Option<String>>()
    }

    /// `Date.prototype.toLocaleString`
    pub fn to_locale_string(&self, locales: Option<&Any>, opts: Option<&Any>) -> Option<String> {
        let mut a = Vec::new();
        if let Some(l) = locales {
            a.push(l.clone());
        }
        if let Some(o) = opts {
            a.push(o.clone());
        }
        self.inner
            .call("toLocaleString", &a)
            .as_::<Option<String>>()
    }
}

impl Date {
    /// Add the given number of milliseconds and return a new `Date`.
    pub fn add_millis(&self, delta: i64) -> Self {
        Date::from_epoch_millis(self.value_of() + delta)
    }

    /// Difference between two dates in milliseconds (`other - self`).
    pub fn diff_millis(&self, other: &Date) -> i64 {
        other.value_of() - self.value_of()
    }
}

impl core::fmt::Display for Date {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&format!("{:?}", self.to_iso_string()))
    }
}
impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.value_of() == other.value_of()
    }
}
impl Eq for Date {}
