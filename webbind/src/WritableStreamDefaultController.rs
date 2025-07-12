use super::*;

#[derive(Clone, Debug)]
pub struct WritableStreamDefaultController {
    inner: emlite::Val,
}
impl FromVal for WritableStreamDefaultController {
    fn from_val(v: &emlite::Val) -> Self {
        WritableStreamDefaultController {
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
impl std::ops::Deref for WritableStreamDefaultController {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WritableStreamDefaultController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WritableStreamDefaultController> for emlite::Val {
    fn from(s: WritableStreamDefaultController) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WritableStreamDefaultController {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }
}
impl WritableStreamDefaultController {
    pub fn error0(&self) -> jsbind::Undefined {
        self.inner.call("error", &[]).as_::<jsbind::Undefined>()
    }

    pub fn error1(&self, e: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("error", &[e.into()])
            .as_::<jsbind::Undefined>()
    }
}
