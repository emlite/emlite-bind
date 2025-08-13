use super::*;




/// The ManagedMediaSource class.
/// [`ManagedMediaSource`](https://developer.mozilla.org/en-US/docs/Web/API/ManagedMediaSource)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ManagedMediaSource {
    inner: MediaSource,
}

impl FromVal for ManagedMediaSource {
    fn from_val(v: &Any) -> Self {
        ManagedMediaSource { inner: MediaSource::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ManagedMediaSource {
    type Target = MediaSource;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ManagedMediaSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ManagedMediaSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ManagedMediaSource {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ManagedMediaSource> for Any {
    fn from(s: ManagedMediaSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ManagedMediaSource> for Any {
    fn from(s: &ManagedMediaSource) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ManagedMediaSource);



impl ManagedMediaSource {
    /// The `new ManagedMediaSource(..)` constructor, creating a new ManagedMediaSource instance
    pub fn new() -> ManagedMediaSource {
        Self {
            inner: Any::global("ManagedMediaSource").new(&[]).as_::<MediaSource>(),
        }
    }

}
impl ManagedMediaSource {
    /// Getter of the `streaming` attribute.
    /// [`ManagedMediaSource.streaming`](https://developer.mozilla.org/en-US/docs/Web/API/ManagedMediaSource/streaming)
    pub fn streaming(&self) -> bool {
        self.inner.get("streaming").as_::<bool>()
    }

}
impl ManagedMediaSource {
    /// Getter of the `onstartstreaming` attribute.
    /// [`ManagedMediaSource.onstartstreaming`](https://developer.mozilla.org/en-US/docs/Web/API/ManagedMediaSource/onstartstreaming)
    pub fn onstartstreaming(&self) -> Any {
        self.inner.get("onstartstreaming").as_::<Any>()
    }

    /// Setter of the `onstartstreaming` attribute.
    /// [`ManagedMediaSource.onstartstreaming`](https://developer.mozilla.org/en-US/docs/Web/API/ManagedMediaSource/onstartstreaming)
    pub fn set_onstartstreaming(&mut self, value: &Any) {
        self.inner.set("onstartstreaming", value);
    }
}
impl ManagedMediaSource {
    /// Getter of the `onendstreaming` attribute.
    /// [`ManagedMediaSource.onendstreaming`](https://developer.mozilla.org/en-US/docs/Web/API/ManagedMediaSource/onendstreaming)
    pub fn onendstreaming(&self) -> Any {
        self.inner.get("onendstreaming").as_::<Any>()
    }

    /// Setter of the `onendstreaming` attribute.
    /// [`ManagedMediaSource.onendstreaming`](https://developer.mozilla.org/en-US/docs/Web/API/ManagedMediaSource/onendstreaming)
    pub fn set_onendstreaming(&mut self, value: &Any) {
        self.inner.set("onendstreaming", value);
    }
}
