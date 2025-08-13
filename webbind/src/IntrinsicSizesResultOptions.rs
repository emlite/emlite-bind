use super::*;




/// The IntrinsicSizesResultOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IntrinsicSizesResultOptions {
    inner: Any,
}

impl FromVal for IntrinsicSizesResultOptions {
    fn from_val(v: &Any) -> Self {
        IntrinsicSizesResultOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IntrinsicSizesResultOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IntrinsicSizesResultOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IntrinsicSizesResultOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IntrinsicSizesResultOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IntrinsicSizesResultOptions> for Any {
    fn from(s: IntrinsicSizesResultOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IntrinsicSizesResultOptions> for Any {
    fn from(s: &IntrinsicSizesResultOptions) -> Any {
        s.inner.clone()
    }
}

impl IntrinsicSizesResultOptions {
    /// Getter of the `maxContentSize` attribute.
    pub fn max_content_size(&self) -> f64 {
        self.inner.get("maxContentSize").as_::<f64>()
    }

    /// Setter of the `maxContentSize` attribute.
    pub fn set_max_content_size(&mut self, value: f64) {
        self.inner.set("maxContentSize", value);
    }
}
impl IntrinsicSizesResultOptions {
    /// Getter of the `minContentSize` attribute.
    pub fn min_content_size(&self) -> f64 {
        self.inner.get("minContentSize").as_::<f64>()
    }

    /// Setter of the `minContentSize` attribute.
    pub fn set_min_content_size(&mut self, value: f64) {
        self.inner.set("minContentSize", value);
    }
}
