use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextTrackCueList {
    inner: emlite::Val,
}
impl FromVal for TextTrackCueList {
    fn from_val(v: &emlite::Val) -> Self {
        TextTrackCueList {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TextTrackCueList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextTrackCueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextTrackCueList> for emlite::Val {
    fn from(s: TextTrackCueList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextTrackCueList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl TextTrackCueList {
    pub fn get_cue_by_id(&self, id: jsbind::DOMString) -> TextTrackCue {
        self.inner
            .call("getCueById", &[id.into()])
            .as_::<TextTrackCue>()
    }
}
