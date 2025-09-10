use super::*;

/// The WaveShaperOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WaveShaperOptions {
    inner: Any,
}

impl FromVal for WaveShaperOptions {
    fn from_val(v: &Any) -> Self {
        WaveShaperOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WaveShaperOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WaveShaperOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WaveShaperOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WaveShaperOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WaveShaperOptions> for Any {
    fn from(s: WaveShaperOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WaveShaperOptions> for Any {
    fn from(s: &WaveShaperOptions) -> Any {
        s.inner.clone()
    }
}

impl WaveShaperOptions {
    /// Getter of the `curve` attribute.
    pub fn curve(&self) -> TypedArray<f32> {
        self.inner.get("curve").as_::<TypedArray<f32>>()
    }

    /// Setter of the `curve` attribute.
    pub fn set_curve(&mut self, value: TypedArray<f32>) {
        self.inner.set("curve", value);
    }
}
impl WaveShaperOptions {
    /// Getter of the `oversample` attribute.
    pub fn oversample(&self) -> OverSampleType {
        self.inner.get("oversample").as_::<OverSampleType>()
    }

    /// Setter of the `oversample` attribute.
    pub fn set_oversample(&mut self, value: &OverSampleType) {
        self.inner.set("oversample", value);
    }
}
