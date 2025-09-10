use super::*;

/// The MLCumulativeSumOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLCumulativeSumOptions {
    inner: Any,
}

impl FromVal for MLCumulativeSumOptions {
    fn from_val(v: &Any) -> Self {
        MLCumulativeSumOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLCumulativeSumOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLCumulativeSumOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLCumulativeSumOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLCumulativeSumOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLCumulativeSumOptions> for Any {
    fn from(s: MLCumulativeSumOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLCumulativeSumOptions> for Any {
    fn from(s: &MLCumulativeSumOptions) -> Any {
        s.inner.clone()
    }
}

impl MLCumulativeSumOptions {
    /// Getter of the `exclusive` attribute.
    pub fn exclusive(&self) -> bool {
        self.inner.get("exclusive").as_::<bool>()
    }

    /// Setter of the `exclusive` attribute.
    pub fn set_exclusive(&mut self, value: bool) {
        self.inner.set("exclusive", value);
    }
}
impl MLCumulativeSumOptions {
    /// Getter of the `reversed` attribute.
    pub fn reversed(&self) -> bool {
        self.inner.get("reversed").as_::<bool>()
    }

    /// Setter of the `reversed` attribute.
    pub fn set_reversed(&mut self, value: bool) {
        self.inner.set("reversed", value);
    }
}
