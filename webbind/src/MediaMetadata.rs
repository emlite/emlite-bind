use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaMetadata {
    inner: emlite::Val,
}
impl FromVal for MediaMetadata {
    fn from_val(v: &emlite::Val) -> Self {
        MediaMetadata {
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
impl core::ops::Deref for MediaMetadata {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaMetadata {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaMetadata {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaMetadata> for emlite::Val {
    fn from(s: MediaMetadata) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MediaMetadata);

impl MediaMetadata {
    pub fn new0() -> MediaMetadata {
        Self {
            inner: emlite::Val::global("MediaMetadata")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: Any) -> MediaMetadata {
        Self {
            inner: emlite::Val::global("MediaMetadata")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl MediaMetadata {
    pub fn title(&self) -> DOMString {
        self.inner.get("title").as_::<DOMString>()
    }

    pub fn set_title(&mut self, value: DOMString) {
        self.inner.set("title", value);
    }
}
impl MediaMetadata {
    pub fn artist(&self) -> DOMString {
        self.inner.get("artist").as_::<DOMString>()
    }

    pub fn set_artist(&mut self, value: DOMString) {
        self.inner.set("artist", value);
    }
}
impl MediaMetadata {
    pub fn album(&self) -> DOMString {
        self.inner.get("album").as_::<DOMString>()
    }

    pub fn set_album(&mut self, value: DOMString) {
        self.inner.set("album", value);
    }
}
impl MediaMetadata {
    pub fn artwork(&self) -> FrozenArray<Object> {
        self.inner.get("artwork").as_::<FrozenArray<Object>>()
    }

    pub fn set_artwork(&mut self, value: FrozenArray<Object>) {
        self.inner.set("artwork", value);
    }
}
impl MediaMetadata {
    pub fn chapter_info(&self) -> FrozenArray<ChapterInformation> {
        self.inner
            .get("chapterInfo")
            .as_::<FrozenArray<ChapterInformation>>()
    }
}
