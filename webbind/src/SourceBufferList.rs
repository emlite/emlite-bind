use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SourceBufferList {
    inner: EventTarget,
}
impl FromVal for SourceBufferList {
    fn from_val(v: &emlite::Val) -> Self {
        SourceBufferList {
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
impl core::ops::Deref for SourceBufferList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SourceBufferList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SourceBufferList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SourceBufferList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SourceBufferList> for emlite::Val {
    fn from(s: SourceBufferList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SourceBufferList> for emlite::Val {
    fn from(s: &SourceBufferList) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SourceBufferList);

impl SourceBufferList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SourceBufferList {
    pub fn onaddsourcebuffer(&self) -> Any {
        self.inner.get("onaddsourcebuffer").as_::<Any>()
    }

    pub fn set_onaddsourcebuffer(&mut self, value: Any) {
        self.inner.set("onaddsourcebuffer", value);
    }
}
impl SourceBufferList {
    pub fn onremovesourcebuffer(&self) -> Any {
        self.inner.get("onremovesourcebuffer").as_::<Any>()
    }

    pub fn set_onremovesourcebuffer(&mut self, value: Any) {
        self.inner.set("onremovesourcebuffer", value);
    }
}
