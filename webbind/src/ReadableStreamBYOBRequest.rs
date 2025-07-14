use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReadableStreamBYOBRequest {
    inner: emlite::Val,
}
impl FromVal for ReadableStreamBYOBRequest {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableStreamBYOBRequest {
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
impl core::ops::Deref for ReadableStreamBYOBRequest {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamBYOBRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ReadableStreamBYOBRequest> for emlite::Val {
    fn from(s: ReadableStreamBYOBRequest) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ReadableStreamBYOBRequest {
    pub fn view(&self) -> jsbind::Any {
        self.inner.get("view").as_::<jsbind::Any>()
    }
}
impl ReadableStreamBYOBRequest {
    pub fn respond(&self, bytes_written: u64) -> jsbind::Undefined {
        self.inner
            .call("respond", &[bytes_written.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl ReadableStreamBYOBRequest {
    pub fn respond_with_new_view(&self, view: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("respondWithNewView", &[view.into()])
            .as_::<jsbind::Undefined>()
    }
}
