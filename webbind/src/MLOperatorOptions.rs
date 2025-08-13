use super::*;




/// The MLOperatorOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLOperatorOptions {
    inner: Any,
}

impl FromVal for MLOperatorOptions {
    fn from_val(v: &Any) -> Self {
        MLOperatorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLOperatorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLOperatorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLOperatorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLOperatorOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLOperatorOptions> for Any {
    fn from(s: MLOperatorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLOperatorOptions> for Any {
    fn from(s: &MLOperatorOptions) -> Any {
        s.inner.clone()
    }
}

impl MLOperatorOptions {
    /// Getter of the `label` attribute.
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
