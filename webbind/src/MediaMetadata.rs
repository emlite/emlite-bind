use super::*;

/// The MediaMetadata class.
/// [`MediaMetadata`](https://developer.mozilla.org/en-US/docs/Web/API/MediaMetadata)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaMetadata {
    inner: Any,
}
impl FromVal for MediaMetadata {
    fn from_val(v: &Any) -> Self {
        MediaMetadata {
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
impl core::ops::Deref for MediaMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaMetadata> for Any {
    fn from(s: MediaMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaMetadata> for Any {
    fn from(s: &MediaMetadata) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaMetadata);

impl MediaMetadata {
    /// The `new MediaMetadata(..)` constructor, creating a new MediaMetadata instance
    pub fn new0() -> MediaMetadata {
        Self {
            inner: Any::global("MediaMetadata").new(&[]).as_::<Any>(),
        }
    }

    /// The `new MediaMetadata(..)` constructor, creating a new MediaMetadata instance
    pub fn new1(init: &Any) -> MediaMetadata {
        Self {
            inner: Any::global("MediaMetadata")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}
impl MediaMetadata {
    /// Getter of the `title` attribute.
    /// [`MediaMetadata.title`](https://developer.mozilla.org/en-US/docs/Web/API/MediaMetadata/title)
    pub fn title(&self) -> DOMString {
        self.inner.get("title").as_::<DOMString>()
    }

    /// Setter of the `title` attribute.
    /// [`MediaMetadata.title`](https://developer.mozilla.org/en-US/docs/Web/API/MediaMetadata/title)
    pub fn set_title(&mut self, value: &DOMString) {
        self.inner.set("title", value);
    }
}
impl MediaMetadata {
    /// Getter of the `artist` attribute.
    /// [`MediaMetadata.artist`](https://developer.mozilla.org/en-US/docs/Web/API/MediaMetadata/artist)
    pub fn artist(&self) -> DOMString {
        self.inner.get("artist").as_::<DOMString>()
    }

    /// Setter of the `artist` attribute.
    /// [`MediaMetadata.artist`](https://developer.mozilla.org/en-US/docs/Web/API/MediaMetadata/artist)
    pub fn set_artist(&mut self, value: &DOMString) {
        self.inner.set("artist", value);
    }
}
impl MediaMetadata {
    /// Getter of the `album` attribute.
    /// [`MediaMetadata.album`](https://developer.mozilla.org/en-US/docs/Web/API/MediaMetadata/album)
    pub fn album(&self) -> DOMString {
        self.inner.get("album").as_::<DOMString>()
    }

    /// Setter of the `album` attribute.
    /// [`MediaMetadata.album`](https://developer.mozilla.org/en-US/docs/Web/API/MediaMetadata/album)
    pub fn set_album(&mut self, value: &DOMString) {
        self.inner.set("album", value);
    }
}
impl MediaMetadata {
    /// Getter of the `artwork` attribute.
    /// [`MediaMetadata.artwork`](https://developer.mozilla.org/en-US/docs/Web/API/MediaMetadata/artwork)
    pub fn artwork(&self) -> FrozenArray<Object> {
        self.inner.get("artwork").as_::<FrozenArray<Object>>()
    }

    /// Setter of the `artwork` attribute.
    /// [`MediaMetadata.artwork`](https://developer.mozilla.org/en-US/docs/Web/API/MediaMetadata/artwork)
    pub fn set_artwork(&mut self, value: &FrozenArray<Object>) {
        self.inner.set("artwork", value);
    }
}
impl MediaMetadata {
    /// Getter of the `chapterInfo` attribute.
    /// [`MediaMetadata.chapterInfo`](https://developer.mozilla.org/en-US/docs/Web/API/MediaMetadata/chapterInfo)
    pub fn chapter_info(&self) -> FrozenArray<ChapterInformation> {
        self.inner
            .get("chapterInfo")
            .as_::<FrozenArray<ChapterInformation>>()
    }
}
