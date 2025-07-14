use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioTrack {
    inner: emlite::Val,
}
impl FromVal for AudioTrack {
    fn from_val(v: &emlite::Val) -> Self {
        AudioTrack {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioTrack {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioTrack> for emlite::Val {
    fn from(s: AudioTrack) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioTrack {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }
}
impl AudioTrack {
    pub fn kind(&self) -> jsbind::DOMString {
        self.inner.get("kind").as_::<jsbind::DOMString>()
    }
}
impl AudioTrack {
    pub fn label(&self) -> jsbind::DOMString {
        self.inner.get("label").as_::<jsbind::DOMString>()
    }
}
impl AudioTrack {
    pub fn language(&self) -> jsbind::DOMString {
        self.inner.get("language").as_::<jsbind::DOMString>()
    }
}
impl AudioTrack {
    pub fn enabled(&self) -> bool {
        self.inner.get("enabled").as_::<bool>()
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.inner.set("enabled", value);
    }
}
impl AudioTrack {
    pub fn source_buffer(&self) -> SourceBuffer {
        self.inner.get("sourceBuffer").as_::<SourceBuffer>()
    }
}
