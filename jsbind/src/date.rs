use crate::Any;
use crate::utils::*;
use alloc::string::String;
use alloc::vec::Vec;

/// One-to-one wrapper around a JS `Date` instance.
///
/// Construction is lazy: no extra fields, just a shared `emlite::Val`.
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
    /// [`Date.prototype.valueOf`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/valueOf)  
    /// (epoch milliseconds).
    #[inline]
    pub fn value_of(&self) -> i64 {
        self.inner.call("valueOf", &[]).as_::<i64>()
    }

    /// [`Date.prototype.getTime`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/getTime)
    #[inline]
    pub fn get_time(&self) -> i64 {
        self.value_of()
    }

    /// [`Date.prototype.toISOString`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)
    pub fn to_iso_string(&self) -> String {
        self.inner.call("toISOString", &[]).as_::<String>()
    }

    /// [`Date.prototype.toUTCString`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toUTCString)
    pub fn to_utc_string(&self) -> String {
        self.inner.call("toUTCString", &[]).as_::<String>()
    }

    /// [`Date.prototype.toLocaleString`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleString)
    pub fn to_locale_string(&self, locales: Option<&Any>, opts: Option<&Any>) -> String {
        let mut a = Vec::new();
        if let Some(l) = locales {
            a.push(l.clone());
        }
        if let Some(o) = opts {
            a.push(o.clone());
        }
        self.inner.call("toLocaleString", &a).as_::<String>()
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
        f.write_str(&self.to_iso_string())
    }
}
impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.value_of() == other.value_of()
    }
}
impl Eq for Date {}
