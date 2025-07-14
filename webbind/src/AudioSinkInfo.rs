use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioSinkInfo {
    inner: emlite::Val,
}
impl FromVal for AudioSinkInfo {
    fn from_val(v: &emlite::Val) -> Self {
        AudioSinkInfo {
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
impl core::ops::Deref for AudioSinkInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioSinkInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioSinkInfo> for emlite::Val {
    fn from(s: AudioSinkInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioSinkInfo {
    pub fn type_(&self) -> AudioSinkType {
        self.inner.get("type").as_::<AudioSinkType>()
    }
}
