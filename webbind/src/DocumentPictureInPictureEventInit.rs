use super::*;

/// The DocumentPictureInPictureEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentPictureInPictureEventInit {
    inner: Any,
}

impl FromVal for DocumentPictureInPictureEventInit {
    fn from_val(v: &Any) -> Self {
        DocumentPictureInPictureEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DocumentPictureInPictureEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DocumentPictureInPictureEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DocumentPictureInPictureEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DocumentPictureInPictureEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DocumentPictureInPictureEventInit> for Any {
    fn from(s: DocumentPictureInPictureEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DocumentPictureInPictureEventInit> for Any {
    fn from(s: &DocumentPictureInPictureEventInit) -> Any {
        s.inner.clone()
    }
}

impl DocumentPictureInPictureEventInit {
    /// Getter of the `window` attribute.
    pub fn window(&self) -> Window {
        self.inner.get("window").as_::<Window>()
    }

    /// Setter of the `window` attribute.
    pub fn set_window(&mut self, value: &Window) {
        self.inner.set("window", value);
    }
}
