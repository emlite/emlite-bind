use super::*;




/// The GainOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GainOptions {
    inner: Any,
}

impl FromVal for GainOptions {
    fn from_val(v: &Any) -> Self {
        GainOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GainOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GainOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GainOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GainOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GainOptions> for Any {
    fn from(s: GainOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GainOptions> for Any {
    fn from(s: &GainOptions) -> Any {
        s.inner.clone()
    }
}

impl GainOptions {
    /// Getter of the `gain` attribute.
    pub fn gain(&self) -> f32 {
        self.inner.get("gain").as_::<f32>()
    }

    /// Setter of the `gain` attribute.
    pub fn set_gain(&mut self, value: f32) {
        self.inner.set("gain", value);
    }
}
