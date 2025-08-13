use super::*;




/// The AudioTrackList class.
/// [`AudioTrackList`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioTrackList {
    inner: EventTarget,
}

impl FromVal for AudioTrackList {
    fn from_val(v: &Any) -> Self {
        AudioTrackList { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioTrackList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioTrackList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioTrackList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioTrackList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AudioTrackList> for Any {
    fn from(s: AudioTrackList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioTrackList> for Any {
    fn from(s: &AudioTrackList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioTrackList);


impl AudioTrackList {
    /// Getter of the `length` attribute.
    /// [`AudioTrackList.length`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl AudioTrackList {
    /// The getTrackById method.
    /// [`AudioTrackList.getTrackById`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/getTrackById)
    pub fn get_track_by_id(&self, id: &JsString) -> AudioTrack {
        self.inner.call("getTrackById", &[id.into(), ]).as_::<AudioTrack>()
    }
}
impl AudioTrackList {
    /// Getter of the `onchange` attribute.
    /// [`AudioTrackList.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`AudioTrackList.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
impl AudioTrackList {
    /// Getter of the `onaddtrack` attribute.
    /// [`AudioTrackList.onaddtrack`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onaddtrack)
    pub fn onaddtrack(&self) -> Any {
        self.inner.get("onaddtrack").as_::<Any>()
    }

    /// Setter of the `onaddtrack` attribute.
    /// [`AudioTrackList.onaddtrack`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onaddtrack)
    pub fn set_onaddtrack(&mut self, value: &Any) {
        self.inner.set("onaddtrack", value);
    }
}
impl AudioTrackList {
    /// Getter of the `onremovetrack` attribute.
    /// [`AudioTrackList.onremovetrack`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onremovetrack)
    pub fn onremovetrack(&self) -> Any {
        self.inner.get("onremovetrack").as_::<Any>()
    }

    /// Setter of the `onremovetrack` attribute.
    /// [`AudioTrackList.onremovetrack`](https://developer.mozilla.org/en-US/docs/Web/API/AudioTrackList/onremovetrack)
    pub fn set_onremovetrack(&mut self, value: &Any) {
        self.inner.set("onremovetrack", value);
    }
}
