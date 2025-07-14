use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioWorklet {
    inner: Worklet,
}
impl FromVal for AudioWorklet {
    fn from_val(v: &emlite::Val) -> Self {
        AudioWorklet {
            inner: Worklet::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioWorklet {
    type Target = Worklet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioWorklet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioWorklet> for emlite::Val {
    fn from(s: AudioWorklet) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioWorklet {
    pub fn port(&self) -> jsbind::Any {
        self.inner.get("port").as_::<jsbind::Any>()
    }
}
