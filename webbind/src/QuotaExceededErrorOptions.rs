use super::*;

/// The QuotaExceededErrorOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct QuotaExceededErrorOptions {
    inner: Any,
}

impl FromVal for QuotaExceededErrorOptions {
    fn from_val(v: &Any) -> Self {
        QuotaExceededErrorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for QuotaExceededErrorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for QuotaExceededErrorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for QuotaExceededErrorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for QuotaExceededErrorOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<QuotaExceededErrorOptions> for Any {
    fn from(s: QuotaExceededErrorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&QuotaExceededErrorOptions> for Any {
    fn from(s: &QuotaExceededErrorOptions) -> Any {
        s.inner.clone()
    }
}

impl QuotaExceededErrorOptions {
    /// Getter of the `quota` attribute.
    pub fn quota(&self) -> f64 {
        self.inner.get("quota").as_::<f64>()
    }

    /// Setter of the `quota` attribute.
    pub fn set_quota(&mut self, value: f64) {
        self.inner.set("quota", value);
    }
}
impl QuotaExceededErrorOptions {
    /// Getter of the `requested` attribute.
    pub fn requested(&self) -> f64 {
        self.inner.get("requested").as_::<f64>()
    }

    /// Setter of the `requested` attribute.
    pub fn set_requested(&mut self, value: f64) {
        self.inner.set("requested", value);
    }
}
