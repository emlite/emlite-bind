use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCompilationMessage {
    inner: emlite::Val,
}
impl FromVal for GPUCompilationMessage {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCompilationMessage {
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
impl core::ops::Deref for GPUCompilationMessage {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCompilationMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUCompilationMessage {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUCompilationMessage {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUCompilationMessage> for emlite::Val {
    fn from(s: GPUCompilationMessage) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUCompilationMessage> for emlite::Val {
    fn from(s: &GPUCompilationMessage) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUCompilationMessage);

impl GPUCompilationMessage {
    pub fn message(&self) -> DOMString {
        self.inner.get("message").as_::<DOMString>()
    }
}
impl GPUCompilationMessage {
    pub fn type_(&self) -> GPUCompilationMessageType {
        self.inner.get("type").as_::<GPUCompilationMessageType>()
    }
}
impl GPUCompilationMessage {
    pub fn line_num(&self) -> u64 {
        self.inner.get("lineNum").as_::<u64>()
    }
}
impl GPUCompilationMessage {
    pub fn line_pos(&self) -> u64 {
        self.inner.get("linePos").as_::<u64>()
    }
}
impl GPUCompilationMessage {
    pub fn offset(&self) -> u64 {
        self.inner.get("offset").as_::<u64>()
    }
}
impl GPUCompilationMessage {
    pub fn length(&self) -> u64 {
        self.inner.get("length").as_::<u64>()
    }
}
