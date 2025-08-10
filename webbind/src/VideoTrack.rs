use super::*;

/// The VideoTrack class.
/// [`VideoTrack`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoTrack {
    inner: Any,
}

impl FromVal for VideoTrack {
    fn from_val(v: &Any) -> Self {
        VideoTrack {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoTrack {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoTrack {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoTrack {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<VideoTrack> for Any {
    fn from(s: VideoTrack) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoTrack> for Any {
    fn from(s: &VideoTrack) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(VideoTrack);

impl VideoTrack {
    /// Getter of the `id` attribute.
    /// [`VideoTrack.id`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}
impl VideoTrack {
    /// Getter of the `kind` attribute.
    /// [`VideoTrack.kind`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/kind)
    pub fn kind(&self) -> JsString {
        self.inner.get("kind").as_::<JsString>()
    }
}
impl VideoTrack {
    /// Getter of the `label` attribute.
    /// [`VideoTrack.label`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }
}
impl VideoTrack {
    /// Getter of the `language` attribute.
    /// [`VideoTrack.language`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/language)
    pub fn language(&self) -> JsString {
        self.inner.get("language").as_::<JsString>()
    }
}
impl VideoTrack {
    /// Getter of the `selected` attribute.
    /// [`VideoTrack.selected`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)
    pub fn selected(&self) -> bool {
        self.inner.get("selected").as_::<bool>()
    }

    /// Setter of the `selected` attribute.
    /// [`VideoTrack.selected`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/selected)
    pub fn set_selected(&mut self, value: bool) {
        self.inner.set("selected", value);
    }
}
impl VideoTrack {
    /// Getter of the `sourceBuffer` attribute.
    /// [`VideoTrack.sourceBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrack/sourceBuffer)
    pub fn source_buffer(&self) -> SourceBuffer {
        self.inner.get("sourceBuffer").as_::<SourceBuffer>()
    }
}
