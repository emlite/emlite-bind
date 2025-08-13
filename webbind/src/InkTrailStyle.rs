use super::*;




/// The InkTrailStyle dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InkTrailStyle {
    inner: Any,
}

impl FromVal for InkTrailStyle {
    fn from_val(v: &Any) -> Self {
        InkTrailStyle { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for InkTrailStyle {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for InkTrailStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for InkTrailStyle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for InkTrailStyle {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<InkTrailStyle> for Any {
    fn from(s: InkTrailStyle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&InkTrailStyle> for Any {
    fn from(s: &InkTrailStyle) -> Any {
        s.inner.clone()
    }
}

impl InkTrailStyle {
    /// Getter of the `color` attribute.
    pub fn color(&self) -> JsString {
        self.inner.get("color").as_::<JsString>()
    }

    /// Setter of the `color` attribute.
    pub fn set_color(&mut self, value: &JsString) {
        self.inner.set("color", value);
    }
}
impl InkTrailStyle {
    /// Getter of the `diameter` attribute.
    pub fn diameter(&self) -> f64 {
        self.inner.get("diameter").as_::<f64>()
    }

    /// Setter of the `diameter` attribute.
    pub fn set_diameter(&mut self, value: f64) {
        self.inner.set("diameter", value);
    }
}
