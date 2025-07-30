use super::*;

/// The TextTrack class.
/// [`TextTrack`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextTrack {
    inner: EventTarget,
}
impl FromVal for TextTrack {
    fn from_val(v: &Any) -> Self {
        TextTrack {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextTrack {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextTrack {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextTrack {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextTrack> for Any {
    fn from(s: TextTrack) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextTrack> for Any {
    fn from(s: &TextTrack) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextTrack);

impl TextTrack {
    /// Getter of the `kind` attribute.
    /// [`TextTrack.kind`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/kind)
    pub fn kind(&self) -> TextTrackKind {
        self.inner.get("kind").as_::<TextTrackKind>()
    }
}
impl TextTrack {
    /// Getter of the `label` attribute.
    /// [`TextTrack.label`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }
}
impl TextTrack {
    /// Getter of the `language` attribute.
    /// [`TextTrack.language`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/language)
    pub fn language(&self) -> JsString {
        self.inner.get("language").as_::<JsString>()
    }
}
impl TextTrack {
    /// Getter of the `id` attribute.
    /// [`TextTrack.id`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}
impl TextTrack {
    /// Getter of the `inBandMetadataTrackDispatchType` attribute.
    /// [`TextTrack.inBandMetadataTrackDispatchType`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/inBandMetadataTrackDispatchType)
    pub fn in_band_metadata_track_dispatch_type(&self) -> JsString {
        self.inner
            .get("inBandMetadataTrackDispatchType")
            .as_::<JsString>()
    }
}
impl TextTrack {
    /// Getter of the `mode` attribute.
    /// [`TextTrack.mode`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)
    pub fn mode(&self) -> TextTrackMode {
        self.inner.get("mode").as_::<TextTrackMode>()
    }

    /// Setter of the `mode` attribute.
    /// [`TextTrack.mode`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/mode)
    pub fn set_mode(&mut self, value: &TextTrackMode) {
        self.inner.set("mode", value);
    }
}
impl TextTrack {
    /// Getter of the `cues` attribute.
    /// [`TextTrack.cues`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/cues)
    pub fn cues(&self) -> TextTrackCueList {
        self.inner.get("cues").as_::<TextTrackCueList>()
    }
}
impl TextTrack {
    /// Getter of the `activeCues` attribute.
    /// [`TextTrack.activeCues`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/activeCues)
    pub fn active_cues(&self) -> TextTrackCueList {
        self.inner.get("activeCues").as_::<TextTrackCueList>()
    }
}
impl TextTrack {
    /// The addCue method.
    /// [`TextTrack.addCue`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/addCue)
    pub fn add_cue(&self, cue: &TextTrackCue) -> Undefined {
        self.inner.call("addCue", &[cue.into()]).as_::<Undefined>()
    }
}
impl TextTrack {
    /// The removeCue method.
    /// [`TextTrack.removeCue`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/removeCue)
    pub fn remove_cue(&self, cue: &TextTrackCue) -> Undefined {
        self.inner
            .call("removeCue", &[cue.into()])
            .as_::<Undefined>()
    }
}
impl TextTrack {
    /// Getter of the `oncuechange` attribute.
    /// [`TextTrack.oncuechange`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)
    pub fn oncuechange(&self) -> Any {
        self.inner.get("oncuechange").as_::<Any>()
    }

    /// Setter of the `oncuechange` attribute.
    /// [`TextTrack.oncuechange`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/oncuechange)
    pub fn set_oncuechange(&mut self, value: &Any) {
        self.inner.set("oncuechange", value);
    }
}
impl TextTrack {
    /// Getter of the `sourceBuffer` attribute.
    /// [`TextTrack.sourceBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrack/sourceBuffer)
    pub fn source_buffer(&self) -> SourceBuffer {
        self.inner.get("sourceBuffer").as_::<SourceBuffer>()
    }
}
