use crate::any::Any;
use crate::utils::*;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

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
    pub fn to_iso_string(&self) -> Option<String> {
        self.inner.call("toISOString", &[]).as_::<Option<String>>()
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
