use super::*;

/// The IsInputPendingOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IsInputPendingOptions {
    inner: Any,
}

impl FromVal for IsInputPendingOptions {
    fn from_val(v: &Any) -> Self {
        IsInputPendingOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IsInputPendingOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IsInputPendingOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IsInputPendingOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IsInputPendingOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IsInputPendingOptions> for Any {
    fn from(s: IsInputPendingOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IsInputPendingOptions> for Any {
    fn from(s: &IsInputPendingOptions) -> Any {
        s.inner.clone()
    }
}

impl IsInputPendingOptions {
    /// Getter of the `includeContinuous` attribute.
    pub fn include_continuous(&self) -> bool {
        self.inner.get("includeContinuous").as_::<bool>()
    }

    /// Setter of the `includeContinuous` attribute.
    pub fn set_include_continuous(&mut self, value: bool) {
        self.inner.set("includeContinuous", value);
    }
}
