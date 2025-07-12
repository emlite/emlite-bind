use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for PushMessageData {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PushMessageData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PushMessageData> for emlite::Val {
    fn from(s: PushMessageData) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PushMessageData {
    pub fn array_buffer(&self) -> jsbind::ArrayBuffer {
        self.inner
            .call("arrayBuffer", &[])
            .as_::<jsbind::ArrayBuffer>()
    }
}
impl PushMessageData {
    pub fn blob(&self) -> Blob {
        self.inner.call("blob", &[]).as_::<Blob>()
    }
}
impl PushMessageData {
    pub fn bytes(&self) -> jsbind::Uint8Array {
        self.inner.call("bytes", &[]).as_::<jsbind::Uint8Array>()
    }
}
impl PushMessageData {
    pub fn json(&self) -> jsbind::Any {
        self.inner.call("json", &[]).as_::<jsbind::Any>()
    }
}
impl PushMessageData {
    pub fn text(&self) -> jsbind::USVString {
        self.inner.call("text", &[]).as_::<jsbind::USVString>()
    }
}
