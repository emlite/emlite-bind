use super::*;

#[derive(Clone, Debug)]
pub struct MediaImage {
    inner: emlite::Val,
}
impl FromVal for MediaImage {
    fn from_val(v: &emlite::Val) -> Self {
        MediaImage { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MediaImage {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaImage> for emlite::Val {
    fn from(s: MediaImage) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaImage {
    pub fn src(&self) -> jsbind::USVString {
        self.inner.get("src").as_::<jsbind::USVString>()
    }

    pub fn set_src(&mut self, value: jsbind::USVString) {
        self.inner.set("src", value);
    }
}
impl MediaImage {
    pub fn sizes(&self) -> jsbind::DOMString {
        self.inner.get("sizes").as_::<jsbind::DOMString>()
    }

    pub fn set_sizes(&mut self, value: jsbind::DOMString) {
        self.inner.set("sizes", value);
    }
}
impl MediaImage {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
#[derive(Clone, Debug)]
pub struct ChapterInformation {
    inner: emlite::Val,
}
impl FromVal for ChapterInformation {
    fn from_val(v: &emlite::Val) -> Self {
        ChapterInformation {
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
impl std::ops::Deref for ChapterInformation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ChapterInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ChapterInformation> for emlite::Val {
    fn from(s: ChapterInformation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ChapterInformation {
    pub fn title(&self) -> jsbind::DOMString {
        self.inner.get("title").as_::<jsbind::DOMString>()
    }
}
impl ChapterInformation {
    pub fn start_time(&self) -> f64 {
        self.inner.get("startTime").as_::<f64>()
    }
}
impl ChapterInformation {
    pub fn artwork(&self) -> jsbind::FrozenArray<MediaImage> {
        self.inner
            .get("artwork")
            .as_::<jsbind::FrozenArray<MediaImage>>()
    }
}
