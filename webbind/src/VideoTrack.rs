use super::*;

#[derive(Clone, Debug)]
pub struct VideoTrack {
    inner: emlite::Val,
}
impl FromVal for VideoTrack {
    fn from_val(v: &emlite::Val) -> Self {
        VideoTrack {
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
impl std::ops::Deref for VideoTrack {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for VideoTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<VideoTrack> for emlite::Val {
    fn from(s: VideoTrack) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl VideoTrack {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }
}
impl VideoTrack {
    pub fn kind(&self) -> jsbind::DOMString {
        self.inner.get("kind").as_::<jsbind::DOMString>()
    }
}
impl VideoTrack {
    pub fn label(&self) -> jsbind::DOMString {
        self.inner.get("label").as_::<jsbind::DOMString>()
    }
}
impl VideoTrack {
    pub fn language(&self) -> jsbind::DOMString {
        self.inner.get("language").as_::<jsbind::DOMString>()
    }
}
impl VideoTrack {
    pub fn selected(&self) -> bool {
        self.inner.get("selected").as_::<bool>()
    }

    pub fn set_selected(&mut self, value: bool) {
        self.inner.set("selected", value);
    }
}
impl VideoTrack {
    pub fn source_buffer(&self) -> SourceBuffer {
        self.inner.get("sourceBuffer").as_::<SourceBuffer>()
    }
}
