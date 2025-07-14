use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamDefaultController {
    inner: emlite::Val,
}
impl FromVal for ReadableStreamDefaultController {
    fn from_val(v: &emlite::Val) -> Self {
        ReadableStreamDefaultController {
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
impl core::ops::Deref for ReadableStreamDefaultController {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamDefaultController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ReadableStreamDefaultController {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ReadableStreamDefaultController {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ReadableStreamDefaultController> for emlite::Val {
    fn from(s: ReadableStreamDefaultController) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ReadableStreamDefaultController);

impl ReadableStreamDefaultController {
    pub fn desired_size(&self) -> f64 {
        self.inner.get("desiredSize").as_::<f64>()
    }
}
impl ReadableStreamDefaultController {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl ReadableStreamDefaultController {
    pub fn enqueue0(&self) -> jsbind::Undefined {
        self.inner.call("enqueue", &[]).as_::<jsbind::Undefined>()
    }

    pub fn enqueue1(&self, chunk: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("enqueue", &[chunk.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl ReadableStreamDefaultController {
    pub fn error0(&self) -> jsbind::Undefined {
        self.inner.call("error", &[]).as_::<jsbind::Undefined>()
    }

    pub fn error1(&self, e: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("error", &[e.into()])
            .as_::<jsbind::Undefined>()
    }
}
