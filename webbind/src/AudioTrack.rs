use super::*;

/// The AudioTrack class.
/// [`AudioTrack`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioTrack {
    inner: Any,
}
impl FromVal for AudioTrack {
    fn from_val(v: &Any) -> Self {
        AudioTrack {
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
impl core::ops::Deref for AudioTrack {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioTrack {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioTrack {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioTrack> for Any {
    fn from(s: AudioTrack) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioTrack> for Any {
    fn from(s: &AudioTrack) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioTrack);

impl AudioTrack {
    /// Getter of the `id` attribute.
    /// [`AudioTrack.id`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}
impl AudioTrack {
    /// Getter of the `kind` attribute.
    /// [`AudioTrack.kind`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/kind)
    pub fn kind(&self) -> JsString {
        self.inner.get("kind").as_::<JsString>()
    }
}
impl AudioTrack {
    /// Getter of the `label` attribute.
    /// [`AudioTrack.label`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }
}
impl AudioTrack {
    /// Getter of the `language` attribute.
    /// [`AudioTrack.language`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/language)
    pub fn language(&self) -> JsString {
        self.inner.get("language").as_::<JsString>()
    }
}
impl AudioTrack {
    /// Getter of the `enabled` attribute.
    /// [`AudioTrack.enabled`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)
    pub fn enabled(&self) -> bool {
        self.inner.get("enabled").as_::<bool>()
    }

    /// Setter of the `enabled` attribute.
    /// [`AudioTrack.enabled`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/enabled)
    pub fn set_enabled(&mut self, value: bool) {
        self.inner.set("enabled", value);
    }
}
impl AudioTrack {
    /// Getter of the `sourceBuffer` attribute.
    /// [`AudioTrack.sourceBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrack/sourceBuffer)
    pub fn source_buffer(&self) -> SourceBuffer {
        self.inner.get("sourceBuffer").as_::<SourceBuffer>()
    }
}
