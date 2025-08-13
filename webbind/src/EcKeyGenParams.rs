use super::*;




/// The EcKeyGenParams dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EcKeyGenParams {
    inner: Any,
}

impl FromVal for EcKeyGenParams {
    fn from_val(v: &Any) -> Self {
        EcKeyGenParams { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EcKeyGenParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EcKeyGenParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EcKeyGenParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EcKeyGenParams {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<EcKeyGenParams> for Any {
    fn from(s: EcKeyGenParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EcKeyGenParams> for Any {
    fn from(s: &EcKeyGenParams) -> Any {
        s.inner.clone()
    }
}

impl EcKeyGenParams {
    /// Getter of the `namedCurve` attribute.
    pub fn named_curve(&self) -> Any {
        self.inner.get("namedCurve").as_::<Any>()
    }

    /// Setter of the `namedCurve` attribute.
    pub fn set_named_curve(&mut self, value: &Any) {
        self.inner.set("namedCurve", value);
    }
}
