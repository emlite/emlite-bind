use super::*;

/// The ImageTrackList class.
/// [`ImageTrackList`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrackList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ImageTrackList {
    inner: Any,
}
impl FromVal for ImageTrackList {
    fn from_val(v: &Any) -> Self {
        ImageTrackList {
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
impl core::ops::Deref for ImageTrackList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ImageTrackList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ImageTrackList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ImageTrackList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ImageTrackList> for Any {
    fn from(s: ImageTrackList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ImageTrackList> for Any {
    fn from(s: &ImageTrackList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ImageTrackList);

impl ImageTrackList {
    /// Getter of the `ready` attribute.
    /// [`ImageTrackList.ready`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrackList/ready)
    pub fn ready(&self) -> Promise<Undefined> {
        self.inner.get("ready").as_::<Promise<Undefined>>()
    }
}
impl ImageTrackList {
    /// Getter of the `length` attribute.
    /// [`ImageTrackList.length`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrackList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl ImageTrackList {
    /// Getter of the `selectedIndex` attribute.
    /// [`ImageTrackList.selectedIndex`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrackList/selectedIndex)
    pub fn selected_index(&self) -> i32 {
        self.inner.get("selectedIndex").as_::<i32>()
    }
}
impl ImageTrackList {
    /// Getter of the `selectedTrack` attribute.
    /// [`ImageTrackList.selectedTrack`](https://developer.mozilla.org/en-US/docs/Web/API/ImageTrackList/selectedTrack)
    pub fn selected_track(&self) -> ImageTrack {
        self.inner.get("selectedTrack").as_::<ImageTrack>()
    }
}
