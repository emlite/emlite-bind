use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioTrackList {
    inner: EventTarget,
}
impl FromVal for AudioTrackList {
    fn from_val(v: &emlite::Val) -> Self {
        AudioTrackList {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for AudioTrackList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioTrackList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioTrackList> for emlite::Val {
    fn from(s: AudioTrackList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AudioTrackList);

impl AudioTrackList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl AudioTrackList {
    pub fn get_track_by_id(&self, id: DOMString) -> AudioTrack {
        self.inner
            .call("getTrackById", &[id.into()])
            .as_::<AudioTrack>()
    }
}
impl AudioTrackList {
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    pub fn set_onchange(&mut self, value: Any) {
        self.inner.set("onchange", value);
    }
}
impl AudioTrackList {
    pub fn onaddtrack(&self) -> Any {
        self.inner.get("onaddtrack").as_::<Any>()
    }

    pub fn set_onaddtrack(&mut self, value: Any) {
        self.inner.set("onaddtrack", value);
    }
}
impl AudioTrackList {
    pub fn onremovetrack(&self) -> Any {
        self.inner.get("onremovetrack").as_::<Any>()
    }

    pub fn set_onremovetrack(&mut self, value: Any) {
        self.inner.set("onremovetrack", value);
    }
}
