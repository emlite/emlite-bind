use super::*;

/// The VideoTrackList class.
/// [`VideoTrackList`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoTrackList {
    inner: EventTarget,
}
impl FromVal for VideoTrackList {
    fn from_val(v: &Any) -> Self {
        VideoTrackList {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VideoTrackList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VideoTrackList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VideoTrackList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VideoTrackList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VideoTrackList> for Any {
    fn from(s: VideoTrackList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VideoTrackList> for Any {
    fn from(s: &VideoTrackList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VideoTrackList);

impl VideoTrackList {
    /// Getter of the `length` attribute.
    /// [`VideoTrackList.length`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl VideoTrackList {
    /// The getTrackById method.
    /// [`VideoTrackList.getTrackById`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/getTrackById)
    pub fn get_track_by_id(&self, id: &DOMString) -> VideoTrack {
        self.inner
            .call("getTrackById", &[id.into()])
            .as_::<VideoTrack>()
    }
}
impl VideoTrackList {
    /// Getter of the `selectedIndex` attribute.
    /// [`VideoTrackList.selectedIndex`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/selectedIndex)
    pub fn selected_index(&self) -> i32 {
        self.inner.get("selectedIndex").as_::<i32>()
    }
}
impl VideoTrackList {
    /// Getter of the `onchange` attribute.
    /// [`VideoTrackList.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`VideoTrackList.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
impl VideoTrackList {
    /// Getter of the `onaddtrack` attribute.
    /// [`VideoTrackList.onaddtrack`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onaddtrack)
    pub fn onaddtrack(&self) -> Any {
        self.inner.get("onaddtrack").as_::<Any>()
    }

    /// Setter of the `onaddtrack` attribute.
    /// [`VideoTrackList.onaddtrack`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onaddtrack)
    pub fn set_onaddtrack(&mut self, value: &Any) {
        self.inner.set("onaddtrack", value);
    }
}
impl VideoTrackList {
    /// Getter of the `onremovetrack` attribute.
    /// [`VideoTrackList.onremovetrack`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onremovetrack)
    pub fn onremovetrack(&self) -> Any {
        self.inner.get("onremovetrack").as_::<Any>()
    }

    /// Setter of the `onremovetrack` attribute.
    /// [`VideoTrackList.onremovetrack`](https://developer.mozilla.org/en-US/docs/Web/API/VideoTrackList/onremovetrack)
    pub fn set_onremovetrack(&mut self, value: &Any) {
        self.inner.set("onremovetrack", value);
    }
}
