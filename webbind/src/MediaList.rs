use super::*;

/// The MediaList class.
/// [`MediaList`](https://developer.mozilla.org/en-US/docs/Web/API/MediaList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaList {
    inner: Any,
}
impl FromVal for MediaList {
    fn from_val(v: &Any) -> Self {
        MediaList {
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
impl core::ops::Deref for MediaList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaList> for Any {
    fn from(s: MediaList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaList> for Any {
    fn from(s: &MediaList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaList);

impl MediaList {
    /// Getter of the `mediaText` attribute.
    /// [`MediaList.mediaText`](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText)
    pub fn media_text(&self) -> CSSOMString {
        self.inner.get("mediaText").as_::<CSSOMString>()
    }

    /// Setter of the `mediaText` attribute.
    /// [`MediaList.mediaText`](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText)
    pub fn set_media_text(&mut self, value: &CSSOMString) {
        self.inner.set("mediaText", value);
    }
}
impl MediaList {
    /// Getter of the `length` attribute.
    /// [`MediaList.length`](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl MediaList {
    /// The item method.
    /// [`MediaList.item`](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/item)
    pub fn item(&self, index: u32) -> CSSOMString {
        self.inner
            .call("item", &[index.into()])
            .as_::<CSSOMString>()
    }
}
impl MediaList {
    /// The appendMedium method.
    /// [`MediaList.appendMedium`](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/appendMedium)
    pub fn append_medium(&self, medium: &CSSOMString) -> Undefined {
        self.inner
            .call("appendMedium", &[medium.into()])
            .as_::<Undefined>()
    }
}
impl MediaList {
    /// The deleteMedium method.
    /// [`MediaList.deleteMedium`](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/deleteMedium)
    pub fn delete_medium(&self, medium: &CSSOMString) -> Undefined {
        self.inner
            .call("deleteMedium", &[medium.into()])
            .as_::<Undefined>()
    }
}
