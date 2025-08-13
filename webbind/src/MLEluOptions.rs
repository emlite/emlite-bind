use super::*;




/// The MLEluOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLEluOptions {
    inner: Any,
}

impl FromVal for MLEluOptions {
    fn from_val(v: &Any) -> Self {
        MLEluOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLEluOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLEluOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLEluOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLEluOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLEluOptions> for Any {
    fn from(s: MLEluOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLEluOptions> for Any {
    fn from(s: &MLEluOptions) -> Any {
        s.inner.clone()
    }
}

impl MLEluOptions {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
