use super::*;




/// The DocumentPictureInPictureOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentPictureInPictureOptions {
    inner: Any,
}

impl FromVal for DocumentPictureInPictureOptions {
    fn from_val(v: &Any) -> Self {
        DocumentPictureInPictureOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DocumentPictureInPictureOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DocumentPictureInPictureOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DocumentPictureInPictureOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DocumentPictureInPictureOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DocumentPictureInPictureOptions> for Any {
    fn from(s: DocumentPictureInPictureOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DocumentPictureInPictureOptions> for Any {
    fn from(s: &DocumentPictureInPictureOptions) -> Any {
        s.inner.clone()
    }
}

impl DocumentPictureInPictureOptions {
    /// Getter of the `width` attribute.
    pub fn width(&self) -> u64 {
        self.inner.get("width").as_::<u64>()
    }

    /// Setter of the `width` attribute.
    pub fn set_width(&mut self, value: u64) {
        self.inner.set("width", value);
    }
}
impl DocumentPictureInPictureOptions {
    /// Getter of the `height` attribute.
    pub fn height(&self) -> u64 {
        self.inner.get("height").as_::<u64>()
    }

    /// Setter of the `height` attribute.
    pub fn set_height(&mut self, value: u64) {
        self.inner.set("height", value);
    }
}
impl DocumentPictureInPictureOptions {
    /// Getter of the `disallowReturnToOpener` attribute.
    pub fn disallow_return_to_opener(&self) -> bool {
        self.inner.get("disallowReturnToOpener").as_::<bool>()
    }

    /// Setter of the `disallowReturnToOpener` attribute.
    pub fn set_disallow_return_to_opener(&mut self, value: bool) {
        self.inner.set("disallowReturnToOpener", value);
    }
}
impl DocumentPictureInPictureOptions {
    /// Getter of the `preferInitialWindowPlacement` attribute.
    pub fn prefer_initial_window_placement(&self) -> bool {
        self.inner.get("preferInitialWindowPlacement").as_::<bool>()
    }

    /// Setter of the `preferInitialWindowPlacement` attribute.
    pub fn set_prefer_initial_window_placement(&mut self, value: bool) {
        self.inner.set("preferInitialWindowPlacement", value);
    }
}
