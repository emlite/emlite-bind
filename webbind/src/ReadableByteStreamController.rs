use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableByteStreamController {
    inner: emlite::Val,
}
impl FromVal for ReadableByteStreamController {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableByteStreamController {
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
impl core::ops::Deref for ReadableByteStreamController {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableByteStreamController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ReadableByteStreamController {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ReadableByteStreamController {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ReadableByteStreamController> for emlite::Val {
    fn from(s: ReadableByteStreamController) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ReadableByteStreamController);

impl ReadableByteStreamController {
    pub fn byob_request(&self) -> ReadableStreamBYOBRequest {
        self.inner
            .get("byobRequest")
            .as_::<ReadableStreamBYOBRequest>()
    }
}
impl ReadableByteStreamController {
    pub fn desired_size(&self) -> f64 {
        self.inner.get("desiredSize").as_::<f64>()
    }
}
impl ReadableByteStreamController {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl ReadableByteStreamController {
    pub fn enqueue(&self, chunk: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("enqueue", &[chunk.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl ReadableByteStreamController {
    pub fn error0(&self) -> jsbind::Undefined {
        self.inner.call("error", &[]).as_::<jsbind::Undefined>()
    }

    pub fn error1(&self, e: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("error", &[e.into()])
            .as_::<jsbind::Undefined>()
    }
}
