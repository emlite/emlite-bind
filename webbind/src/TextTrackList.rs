use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextTrackList {
    inner: EventTarget,
}
impl FromVal for TextTrackList {
    fn from_val(v: &emlite::Val) -> Self {
        TextTrackList {
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
impl AsRef<emlite::Val> for TextTrackList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TextTrackList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TextTrackList> for emlite::Val {
    fn from(s: TextTrackList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TextTrackList);

impl TextTrackList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl TextTrackList {
    pub fn get_track_by_id(&self, id: jsbind::DOMString) -> TextTrack {
        self.inner
            .call("getTrackById", &[id.into()])
            .as_::<TextTrack>()
    }
}
impl TextTrackList {
    pub fn onchange(&self) -> jsbind::Any {
        self.inner.get("onchange").as_::<jsbind::Any>()
    }

    pub fn set_onchange(&mut self, value: jsbind::Any) {
        self.inner.set("onchange", value);
    }
}
impl TextTrackList {
    pub fn onaddtrack(&self) -> jsbind::Any {
        self.inner.get("onaddtrack").as_::<jsbind::Any>()
    }

    pub fn set_onaddtrack(&mut self, value: jsbind::Any) {
        self.inner.set("onaddtrack", value);
    }
}
impl TextTrackList {
    pub fn onremovetrack(&self) -> jsbind::Any {
        self.inner.get("onremovetrack").as_::<jsbind::Any>()
    }

    pub fn set_onremovetrack(&mut self, value: jsbind::Any) {
        self.inner.set("onremovetrack", value);
    }
}
