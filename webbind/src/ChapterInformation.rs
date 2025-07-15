use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for MediaImage {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaImage {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaImage {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaImage> for emlite::Val {
    fn from(s: MediaImage) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaImage> for emlite::Val {
    fn from(s: &MediaImage) -> emlite::Val {
        s.inner.clone()
    }
}

impl MediaImage {
    pub fn src(&self) -> USVString {
        self.inner.get("src").as_::<USVString>()
    }

    pub fn set_src(&mut self, value: USVString) {
        self.inner.set("src", value);
    }
}
impl MediaImage {
    pub fn sizes(&self) -> DOMString {
        self.inner.get("sizes").as_::<DOMString>()
    }

    pub fn set_sizes(&mut self, value: DOMString) {
        self.inner.set("sizes", value);
    }
}
impl MediaImage {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: DOMString) {
        self.inner.set("type", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for ChapterInformation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ChapterInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ChapterInformation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ChapterInformation {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ChapterInformation> for emlite::Val {
    fn from(s: ChapterInformation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ChapterInformation> for emlite::Val {
    fn from(s: &ChapterInformation) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ChapterInformation);

impl ChapterInformation {
    pub fn title(&self) -> DOMString {
        self.inner.get("title").as_::<DOMString>()
    }
}
impl ChapterInformation {
    pub fn start_time(&self) -> f64 {
        self.inner.get("startTime").as_::<f64>()
    }
}
impl ChapterInformation {
    pub fn artwork(&self) -> FrozenArray<MediaImage> {
        self.inner.get("artwork").as_::<FrozenArray<MediaImage>>()
    }
}
