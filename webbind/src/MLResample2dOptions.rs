use super::*;




/// The MLResample2dOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLResample2dOptions {
    inner: Any,
}

impl FromVal for MLResample2dOptions {
    fn from_val(v: &Any) -> Self {
        MLResample2dOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLResample2dOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLResample2dOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLResample2dOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLResample2dOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLResample2dOptions> for Any {
    fn from(s: MLResample2dOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLResample2dOptions> for Any {
    fn from(s: &MLResample2dOptions) -> Any {
        s.inner.clone()
    }
}

impl MLResample2dOptions {
    /// Getter of the `mode` attribute.
    pub fn mode(&self) -> MLInterpolationMode {
        self.inner.get("mode").as_::<MLInterpolationMode>()
    }

    /// Setter of the `mode` attribute.
    pub fn set_mode(&mut self, value: &MLInterpolationMode) {
        self.inner.set("mode", value);
    }
}
impl MLResample2dOptions {
    /// Getter of the `scales` attribute.
    pub fn scales(&self) -> TypedArray<f32> {
        self.inner.get("scales").as_::<TypedArray<f32>>()
    }

    /// Setter of the `scales` attribute.
    pub fn set_scales(&mut self, value: TypedArray<f32>) {
        self.inner.set("scales", value);
    }
}
impl MLResample2dOptions {
    /// Getter of the `sizes` attribute.
    pub fn sizes(&self) -> TypedArray<u32> {
        self.inner.get("sizes").as_::<TypedArray<u32>>()
    }

    /// Setter of the `sizes` attribute.
    pub fn set_sizes(&mut self, value: TypedArray<u32>) {
        self.inner.set("sizes", value);
    }
}
impl MLResample2dOptions {
    /// Getter of the `axes` attribute.
    pub fn axes(&self) -> TypedArray<u32> {
        self.inner.get("axes").as_::<TypedArray<u32>>()
    }

    /// Setter of the `axes` attribute.
    pub fn set_axes(&mut self, value: TypedArray<u32>) {
        self.inner.set("axes", value);
    }
}
