use super::*;

/// The QuotaExceededError class.
/// [`QuotaExceededError`](https://developer.mozilla.org/en-US/docs/Web/API/QuotaExceededError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct QuotaExceededError {
    inner: DOMException,
}

impl FromVal for QuotaExceededError {
    fn from_val(v: &Any) -> Self {
        QuotaExceededError {
            inner: DOMException::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for QuotaExceededError {
    type Target = DOMException;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for QuotaExceededError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for QuotaExceededError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for QuotaExceededError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<QuotaExceededError> for Any {
    fn from(s: QuotaExceededError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&QuotaExceededError> for Any {
    fn from(s: &QuotaExceededError) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(QuotaExceededError);

impl QuotaExceededError {
    /// The `new QuotaExceededError(..)` constructor, creating a new QuotaExceededError instance
    pub fn new0() -> QuotaExceededError {
        Self {
            inner: Any::global("QuotaExceededError")
                .new(&[])
                .as_::<DOMException>(),
        }
    }

    /// The `new QuotaExceededError(..)` constructor, creating a new QuotaExceededError instance
    pub fn new1(message: &JsString) -> QuotaExceededError {
        Self {
            inner: Any::global("QuotaExceededError")
                .new(&[message.into()])
                .as_::<DOMException>(),
        }
    }

    /// The `new QuotaExceededError(..)` constructor, creating a new QuotaExceededError instance
    pub fn new2(message: &JsString, options: &QuotaExceededErrorOptions) -> QuotaExceededError {
        Self {
            inner: Any::global("QuotaExceededError")
                .new(&[message.into(), options.into()])
                .as_::<DOMException>(),
        }
    }
}
impl QuotaExceededError {
    /// Getter of the `quota` attribute.
    /// [`QuotaExceededError.quota`](https://developer.mozilla.org/en-US/docs/Web/API/QuotaExceededError/quota)
    pub fn quota(&self) -> f64 {
        self.inner.get("quota").as_::<f64>()
    }
}
impl QuotaExceededError {
    /// Getter of the `requested` attribute.
    /// [`QuotaExceededError.requested`](https://developer.mozilla.org/en-US/docs/Web/API/QuotaExceededError/requested)
    pub fn requested(&self) -> f64 {
        self.inner.get("requested").as_::<f64>()
    }
}
