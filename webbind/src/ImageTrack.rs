use super::*;




/// The ImageTrack class.
/// [`ImageTrack`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrack)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageTrack {
    inner: Any,
}

impl FromVal for ImageTrack {
    fn from_val(v: &Any) -> Self {
        ImageTrack { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ImageTrack {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ImageTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ImageTrack {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ImageTrack {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ImageTrack> for Any {
    fn from(s: ImageTrack) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ImageTrack> for Any {
    fn from(s: &ImageTrack) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ImageTrack);


impl ImageTrack {
    /// Getter of the `animated` attribute.
    /// [`ImageTrack.animated`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrack/animated)
    pub fn animated(&self) -> bool {
        self.inner.get("animated").as_::<bool>()
    }

}
impl ImageTrack {
    /// Getter of the `frameCount` attribute.
    /// [`ImageTrack.frameCount`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrack/frameCount)
    pub fn frame_count(&self) -> u32 {
        self.inner.get("frameCount").as_::<u32>()
    }

}
impl ImageTrack {
    /// Getter of the `repetitionCount` attribute.
    /// [`ImageTrack.repetitionCount`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrack/repetitionCount)
    pub fn repetition_count(&self) -> f32 {
        self.inner.get("repetitionCount").as_::<f32>()
    }

}
impl ImageTrack {
    /// Getter of the `selected` attribute.
    /// [`ImageTrack.selected`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrack/selected)
    pub fn selected(&self) -> bool {
        self.inner.get("selected").as_::<bool>()
    }

    /// Setter of the `selected` attribute.
    /// [`ImageTrack.selected`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrack/selected)
    pub fn set_selected(&mut self, value: bool) {
        self.inner.set("selected", value);
    }
}
