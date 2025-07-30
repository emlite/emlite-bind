use super::*;

/// The TextTrackList class.
/// [`TextTrackList`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextTrackList {
    inner: EventTarget,
}
impl FromVal for TextTrackList {
    fn from_val(v: &Any) -> Self {
        TextTrackList {
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
impl core::ops::Deref for TextTrackList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextTrackList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextTrackList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextTrackList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextTrackList> for Any {
    fn from(s: TextTrackList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextTrackList> for Any {
    fn from(s: &TextTrackList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextTrackList);

impl TextTrackList {
    /// Getter of the `length` attribute.
    /// [`TextTrackList.length`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl TextTrackList {
    /// The getTrackById method.
    /// [`TextTrackList.getTrackById`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/getTrackById)
    pub fn get_track_by_id(&self, id: &JsString) -> TextTrack {
        self.inner
            .call("getTrackById", &[id.into()])
            .as_::<TextTrack>()
    }
}
impl TextTrackList {
    /// Getter of the `onchange` attribute.
    /// [`TextTrackList.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`TextTrackList.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
impl TextTrackList {
    /// Getter of the `onaddtrack` attribute.
    /// [`TextTrackList.onaddtrack`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onaddtrack)
    pub fn onaddtrack(&self) -> Any {
        self.inner.get("onaddtrack").as_::<Any>()
    }

    /// Setter of the `onaddtrack` attribute.
    /// [`TextTrackList.onaddtrack`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onaddtrack)
    pub fn set_onaddtrack(&mut self, value: &Any) {
        self.inner.set("onaddtrack", value);
    }
}
impl TextTrackList {
    /// Getter of the `onremovetrack` attribute.
    /// [`TextTrackList.onremovetrack`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onremovetrack)
    pub fn onremovetrack(&self) -> Any {
        self.inner.get("onremovetrack").as_::<Any>()
    }

    /// Setter of the `onremovetrack` attribute.
    /// [`TextTrackList.onremovetrack`](https://developer.mozilla.org/en-US/docs/Web/API/TextTrackList/onremovetrack)
    pub fn set_onremovetrack(&mut self, value: &Any) {
        self.inner.set("onremovetrack", value);
    }
}
