use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextTrackCue {
    inner: EventTarget,
}
impl FromVal for TextTrackCue {
    fn from_val(v: &emlite::Val) -> Self {
        TextTrackCue {
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
impl core::ops::Deref for TextTrackCue {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextTrackCue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TextTrackCue {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TextTrackCue {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TextTrackCue> for emlite::Val {
    fn from(s: TextTrackCue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TextTrackCue);

impl TextTrackCue {
    pub fn track(&self) -> TextTrack {
        self.inner.get("track").as_::<TextTrack>()
    }
}
impl TextTrackCue {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }

    pub fn set_id(&mut self, value: jsbind::DOMString) {
        self.inner.set("id", value);
    }
}
impl TextTrackCue {
    pub fn start_time(&self) -> f64 {
        self.inner.get("startTime").as_::<f64>()
    }

    pub fn set_start_time(&mut self, value: f64) {
        self.inner.set("startTime", value);
    }
}
impl TextTrackCue {
    pub fn end_time(&self) -> f64 {
        self.inner.get("endTime").as_::<f64>()
    }

    pub fn set_end_time(&mut self, value: f64) {
        self.inner.set("endTime", value);
    }
}
impl TextTrackCue {
    pub fn pause_on_exit(&self) -> bool {
        self.inner.get("pauseOnExit").as_::<bool>()
    }

    pub fn set_pause_on_exit(&mut self, value: bool) {
        self.inner.set("pauseOnExit", value);
    }
}
impl TextTrackCue {
    pub fn onenter(&self) -> jsbind::Any {
        self.inner.get("onenter").as_::<jsbind::Any>()
    }

    pub fn set_onenter(&mut self, value: jsbind::Any) {
        self.inner.set("onenter", value);
    }
}
impl TextTrackCue {
    pub fn onexit(&self) -> jsbind::Any {
        self.inner.get("onexit").as_::<jsbind::Any>()
    }

    pub fn set_onexit(&mut self, value: jsbind::Any) {
        self.inner.set("onexit", value);
    }
}
