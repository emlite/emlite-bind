use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for ManagedSourceBuffer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ManagedSourceBuffer {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&ManagedSourceBuffer> for emlite::Val {
    fn from(s: &ManagedSourceBuffer) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ManagedSourceBuffer);

impl ManagedSourceBuffer {
    pub fn onbufferedchange(&self) -> Any {
        self.inner.get("onbufferedchange").as_::<Any>()
    }

    pub fn set_onbufferedchange(&mut self, value: &Any) {
        self.inner.set("onbufferedchange", value);
    }
}
