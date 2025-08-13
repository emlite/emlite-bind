use super::*;




/// The HMACGetSecretOutput dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HMACGetSecretOutput {
    inner: Any,
}

impl FromVal for HMACGetSecretOutput {
    fn from_val(v: &Any) -> Self {
        HMACGetSecretOutput { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HMACGetSecretOutput {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HMACGetSecretOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HMACGetSecretOutput {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HMACGetSecretOutput {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HMACGetSecretOutput> for Any {
    fn from(s: HMACGetSecretOutput) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HMACGetSecretOutput> for Any {
    fn from(s: &HMACGetSecretOutput) -> Any {
        s.inner.clone()
    }
}

impl HMACGetSecretOutput {
    /// Getter of the `output1` attribute.
    pub fn output1(&self) -> ArrayBuffer {
        self.inner.get("output1").as_::<ArrayBuffer>()
    }

    /// Setter of the `output1` attribute.
    pub fn set_output1(&mut self, value: &ArrayBuffer) {
        self.inner.set("output1", value);
    }
}
impl HMACGetSecretOutput {
    /// Getter of the `output2` attribute.
    pub fn output2(&self) -> ArrayBuffer {
        self.inner.get("output2").as_::<ArrayBuffer>()
    }

    /// Setter of the `output2` attribute.
    pub fn set_output2(&mut self, value: &ArrayBuffer) {
        self.inner.set("output2", value);
    }
}
