use super::*;




/// The StereoPannerOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StereoPannerOptions {
    inner: Any,
}

impl FromVal for StereoPannerOptions {
    fn from_val(v: &Any) -> Self {
        StereoPannerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StereoPannerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StereoPannerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StereoPannerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StereoPannerOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<StereoPannerOptions> for Any {
    fn from(s: StereoPannerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StereoPannerOptions> for Any {
    fn from(s: &StereoPannerOptions) -> Any {
        s.inner.clone()
    }
}

impl StereoPannerOptions {
    /// Getter of the `pan` attribute.
    pub fn pan(&self) -> f32 {
        self.inner.get("pan").as_::<f32>()
    }

    /// Setter of the `pan` attribute.
    pub fn set_pan(&mut self, value: f32) {
        self.inner.set("pan", value);
    }
}
