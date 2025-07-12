use super::*;

#[derive(Clone, Debug)]
pub struct TransformStreamDefaultController {
    inner: emlite::Val,
}
impl FromVal for TransformStreamDefaultController {
    fn from_val(v: &emlite::Val) -> Self {
        TransformStreamDefaultController {
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
impl std::ops::Deref for TransformStreamDefaultController {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TransformStreamDefaultController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TransformStreamDefaultController> for emlite::Val {
    fn from(s: TransformStreamDefaultController) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TransformStreamDefaultController {
    pub fn desired_size(&self) -> f64 {
        self.inner.get("desiredSize").as_::<f64>()
    }
}
impl TransformStreamDefaultController {
    pub fn enqueue0(&self) -> jsbind::Undefined {
        self.inner.call("enqueue", &[]).as_::<jsbind::Undefined>()
    }

    pub fn enqueue1(&self, chunk: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("enqueue", &[chunk.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl TransformStreamDefaultController {
    pub fn error0(&self) -> jsbind::Undefined {
        self.inner.call("error", &[]).as_::<jsbind::Undefined>()
    }

    pub fn error1(&self, reason: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("error", &[reason.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl TransformStreamDefaultController {
    pub fn terminate(&self) -> jsbind::Undefined {
        self.inner.call("terminate", &[]).as_::<jsbind::Undefined>()
    }
}
