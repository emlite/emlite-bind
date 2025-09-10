use super::*;

/// The KeyframeEffectOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeyframeEffectOptions {
    inner: Any,
}

impl FromVal for KeyframeEffectOptions {
    fn from_val(v: &Any) -> Self {
        KeyframeEffectOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for KeyframeEffectOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KeyframeEffectOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KeyframeEffectOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KeyframeEffectOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<KeyframeEffectOptions> for Any {
    fn from(s: KeyframeEffectOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KeyframeEffectOptions> for Any {
    fn from(s: &KeyframeEffectOptions) -> Any {
        s.inner.clone()
    }
}

impl KeyframeEffectOptions {
    /// Getter of the `composite` attribute.
    pub fn composite(&self) -> CompositeOperation {
        self.inner.get("composite").as_::<CompositeOperation>()
    }

    /// Setter of the `composite` attribute.
    pub fn set_composite(&mut self, value: &CompositeOperation) {
        self.inner.set("composite", value);
    }
}
impl KeyframeEffectOptions {
    /// Getter of the `pseudoElement` attribute.
    pub fn pseudo_element(&self) -> JsString {
        self.inner.get("pseudoElement").as_::<JsString>()
    }

    /// Setter of the `pseudoElement` attribute.
    pub fn set_pseudo_element(&mut self, value: &JsString) {
        self.inner.set("pseudoElement", value);
    }
}
