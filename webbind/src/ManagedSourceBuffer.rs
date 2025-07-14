use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ManagedSourceBuffer {
    inner: SourceBuffer,
}
impl FromVal for ManagedSourceBuffer {
    fn from_val(v: &emlite::Val) -> Self {
        ManagedSourceBuffer {
            inner: SourceBuffer::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ManagedSourceBuffer {
    type Target = SourceBuffer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ManagedSourceBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ManagedSourceBuffer> for emlite::Val {
    fn from(s: ManagedSourceBuffer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ManagedSourceBuffer {
    pub fn onbufferedchange(&self) -> jsbind::Any {
        self.inner.get("onbufferedchange").as_::<jsbind::Any>()
    }

    pub fn set_onbufferedchange(&mut self, value: jsbind::Any) {
        self.inner.set("onbufferedchange", value);
    }
}
