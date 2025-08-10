use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaMetadataInit {
    inner: Any,
}
impl FromVal for MediaMetadataInit {
    fn from_val(v: &Any) -> Self {
        MediaMetadataInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaMetadataInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaMetadataInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaMetadataInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaMetadataInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaMetadataInit> for Any {
    fn from(s: MediaMetadataInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaMetadataInit> for Any {
    fn from(s: &MediaMetadataInit) -> Any {
        s.inner.clone()
    }
}

impl MediaMetadataInit {
    pub fn title(&self) -> JsString {
        self.inner.get("title").as_::<JsString>()
    }

    pub fn set_title(&mut self, value: &JsString) {
        self.inner.set("title", value);
    }
}
impl MediaMetadataInit {
    pub fn artist(&self) -> JsString {
        self.inner.get("artist").as_::<JsString>()
    }

    pub fn set_artist(&mut self, value: &JsString) {
        self.inner.set("artist", value);
    }
}
impl MediaMetadataInit {
    pub fn album(&self) -> JsString {
        self.inner.get("album").as_::<JsString>()
    }

    pub fn set_album(&mut self, value: &JsString) {
        self.inner.set("album", value);
    }
}
impl MediaMetadataInit {
    pub fn artwork(&self) -> TypedArray<MediaImage> {
        self.inner.get("artwork").as_::<TypedArray<MediaImage>>()
    }

    pub fn set_artwork(&mut self, value: &TypedArray<MediaImage>) {
        self.inner.set("artwork", value);
    }
}
impl MediaMetadataInit {
    pub fn chapter_info(&self) -> TypedArray<ChapterInformationInit> {
        self.inner
            .get("chapterInfo")
            .as_::<TypedArray<ChapterInformationInit>>()
    }

    pub fn set_chapter_info(&mut self, value: &TypedArray<ChapterInformationInit>) {
        self.inner.set("chapterInfo", value);
    }
}
