use super::*;

/// The MLLayerNormalizationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLLayerNormalizationOptions {
    inner: Any,
}

impl FromVal for MLLayerNormalizationOptions {
    fn from_val(v: &Any) -> Self {
        MLLayerNormalizationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLLayerNormalizationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLLayerNormalizationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLLayerNormalizationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLLayerNormalizationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLLayerNormalizationOptions> for Any {
    fn from(s: MLLayerNormalizationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLLayerNormalizationOptions> for Any {
    fn from(s: &MLLayerNormalizationOptions) -> Any {
        s.inner.clone()
    }
}

impl MLLayerNormalizationOptions {
    /// Getter of the `scale` attribute.
    pub fn scale(&self) -> MLOperand {
        self.inner.get("scale").as_::<MLOperand>()
    }

    /// Setter of the `scale` attribute.
    pub fn set_scale(&mut self, value: &MLOperand) {
        self.inner.set("scale", value);
    }
}
impl MLLayerNormalizationOptions {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLLayerNormalizationOptions {
    /// Getter of the `axes` attribute.
    pub fn axes(&self) -> TypedArray<u32> {
        self.inner.get("axes").as_::<TypedArray<u32>>()
    }

    /// Setter of the `axes` attribute.
    pub fn set_axes(&mut self, value: TypedArray<u32>) {
        self.inner.set("axes", value);
    }
}
impl MLLayerNormalizationOptions {
    /// Getter of the `epsilon` attribute.
    pub fn epsilon(&self) -> f64 {
        self.inner.get("epsilon").as_::<f64>()
    }

    /// Setter of the `epsilon` attribute.
    pub fn set_epsilon(&mut self, value: f64) {
        self.inner.set("epsilon", value);
    }
}
