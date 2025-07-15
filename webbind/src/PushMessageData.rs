use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PushMessageData {
    inner: emlite::Val,
}
impl FromVal for PushMessageData {
    fn from_val(v: &emlite::Val) -> Self {
        PushMessageData {
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
impl core::ops::Deref for PushMessageData {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PushMessageData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PushMessageData {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PushMessageData {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PushMessageData> for emlite::Val {
    fn from(s: PushMessageData) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PushMessageData> for emlite::Val {
    fn from(s: &PushMessageData) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PushMessageData);

impl PushMessageData {
    pub fn array_buffer(&self) -> ArrayBuffer {
        self.inner.call("arrayBuffer", &[]).as_::<ArrayBuffer>()
    }
}
impl PushMessageData {
    pub fn blob(&self) -> Blob {
        self.inner.call("blob", &[]).as_::<Blob>()
    }
}
impl PushMessageData {
    pub fn bytes(&self) -> Uint8Array {
        self.inner.call("bytes", &[]).as_::<Uint8Array>()
    }
}
impl PushMessageData {
    pub fn json(&self) -> Any {
        self.inner.call("json", &[]).as_::<Any>()
    }
}
impl PushMessageData {
    pub fn text(&self) -> String {
        self.inner.call("text", &[]).as_::<String>()
    }
}
