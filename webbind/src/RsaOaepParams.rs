use super::*;




/// The RsaOaepParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RsaOaepParams {
    inner: Any,
}

impl FromVal for RsaOaepParams {
    fn from_val(v: &Any) -> Self {
        RsaOaepParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RsaOaepParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RsaOaepParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RsaOaepParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RsaOaepParams {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RsaOaepParams> for Any {
    fn from(s: RsaOaepParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RsaOaepParams> for Any {
    fn from(s: &RsaOaepParams) -> Any {
        s.inner.clone()
    }
}

impl RsaOaepParams {
    /// Getter of the `label` attribute.
    pub fn label(&self) -> Any {
        self.inner.get("label").as_::<Any>()
    }

    /// Setter of the `label` attribute.
    pub fn set_label(&mut self, value: &Any) {
        self.inner.set("label", value);
    }
}
