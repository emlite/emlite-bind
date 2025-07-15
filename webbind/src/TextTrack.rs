use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextTrack {
    inner: EventTarget,
}
impl FromVal for TextTrack {
    fn from_val(v: &emlite::Val) -> Self {
        TextTrack {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for TextTrack {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TextTrack {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TextTrack> for emlite::Val {
    fn from(s: TextTrack) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&TextTrack> for emlite::Val {
    fn from(s: &TextTrack) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextTrack);

impl TextTrack {
    pub fn kind(&self) -> TextTrackKind {
        self.inner.get("kind").as_::<TextTrackKind>()
    }
}
impl TextTrack {
    pub fn label(&self) -> DOMString {
        self.inner.get("label").as_::<DOMString>()
    }
}
impl TextTrack {
    pub fn language(&self) -> DOMString {
        self.inner.get("language").as_::<DOMString>()
    }
}
impl TextTrack {
    pub fn id(&self) -> DOMString {
        self.inner.get("id").as_::<DOMString>()
    }
}
impl TextTrack {
    pub fn in_band_metadata_track_dispatch_type(&self) -> DOMString {
        self.inner
            .get("inBandMetadataTrackDispatchType")
            .as_::<DOMString>()
    }
}
impl TextTrack {
    pub fn mode(&self) -> TextTrackMode {
        self.inner.get("mode").as_::<TextTrackMode>()
    }

    pub fn set_mode(&mut self, value: TextTrackMode) {
        self.inner.set("mode", value);
    }
}
impl TextTrack {
    pub fn cues(&self) -> TextTrackCueList {
        self.inner.get("cues").as_::<TextTrackCueList>()
    }
}
impl TextTrack {
    pub fn active_cues(&self) -> TextTrackCueList {
        self.inner.get("activeCues").as_::<TextTrackCueList>()
    }
}
impl TextTrack {
    pub fn add_cue(&self, cue: TextTrackCue) -> Undefined {
        self.inner.call("addCue", &[cue.into()]).as_::<Undefined>()
    }
}
impl TextTrack {
    pub fn remove_cue(&self, cue: TextTrackCue) -> Undefined {
        self.inner
            .call("removeCue", &[cue.into()])
            .as_::<Undefined>()
    }
}
impl TextTrack {
    pub fn oncuechange(&self) -> Any {
        self.inner.get("oncuechange").as_::<Any>()
    }

    pub fn set_oncuechange(&mut self, value: Any) {
        self.inner.set("oncuechange", value);
    }
}
impl TextTrack {
    pub fn source_buffer(&self) -> SourceBuffer {
        self.inner.get("sourceBuffer").as_::<SourceBuffer>()
    }
}
