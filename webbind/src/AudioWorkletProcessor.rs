use super::*;

#[derive(Clone, Debug)]
pub struct AudioWorkletProcessor {
    inner: emlite::Val,
}
impl FromVal for AudioWorkletProcessor {
    fn from_val(v: &emlite::Val) -> Self {
        AudioWorkletProcessor {
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
impl std::ops::Deref for AudioWorkletProcessor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioWorkletProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioWorkletProcessor> for emlite::Val {
    fn from(s: AudioWorkletProcessor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioWorkletProcessor {
    pub fn new() -> AudioWorkletProcessor {
        Self {
            inner: emlite::Val::global("AudioWorkletProcessor")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl AudioWorkletProcessor {
    pub fn port(&self) -> jsbind::Any {
        self.inner.get("port").as_::<jsbind::Any>()
    }
}
