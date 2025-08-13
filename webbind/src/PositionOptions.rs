use super::*;




/// The PositionOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PositionOptions {
    inner: Any,
}

impl FromVal for PositionOptions {
    fn from_val(v: &Any) -> Self {
        PositionOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PositionOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PositionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PositionOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PositionOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PositionOptions> for Any {
    fn from(s: PositionOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PositionOptions> for Any {
    fn from(s: &PositionOptions) -> Any {
        s.inner.clone()
    }
}

impl PositionOptions {
    /// Getter of the `enableHighAccuracy` attribute.
    pub fn enable_high_accuracy(&self) -> bool {
        self.inner.get("enableHighAccuracy").as_::<bool>()
    }

    /// Setter of the `enableHighAccuracy` attribute.
    pub fn set_enable_high_accuracy(&mut self, value: bool) {
        self.inner.set("enableHighAccuracy", value);
    }
}
impl PositionOptions {
    /// Getter of the `timeout` attribute.
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    /// Setter of the `timeout` attribute.
    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl PositionOptions {
    /// Getter of the `maximumAge` attribute.
    pub fn maximum_age(&self) -> u32 {
        self.inner.get("maximumAge").as_::<u32>()
    }

    /// Setter of the `maximumAge` attribute.
    pub fn set_maximum_age(&mut self, value: u32) {
        self.inner.set("maximumAge", value);
    }
}
