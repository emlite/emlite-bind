use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for TransformStreamDefaultController {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TransformStreamDefaultController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TransformStreamDefaultController {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TransformStreamDefaultController {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TransformStreamDefaultController> for emlite::Val {
    fn from(s: TransformStreamDefaultController) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TransformStreamDefaultController);

impl TransformStreamDefaultController {
    pub fn desired_size(&self) -> f64 {
        self.inner.get("desiredSize").as_::<f64>()
    }
}
impl TransformStreamDefaultController {
    pub fn enqueue0(&self) -> Undefined {
        self.inner.call("enqueue", &[]).as_::<Undefined>()
    }

    pub fn enqueue1(&self, chunk: Any) -> Undefined {
        self.inner
            .call("enqueue", &[chunk.into()])
            .as_::<Undefined>()
    }
}
impl TransformStreamDefaultController {
    pub fn error0(&self) -> Undefined {
        self.inner.call("error", &[]).as_::<Undefined>()
    }

    pub fn error1(&self, reason: Any) -> Undefined {
        self.inner
            .call("error", &[reason.into()])
            .as_::<Undefined>()
    }
}
impl TransformStreamDefaultController {
    pub fn terminate(&self) -> Undefined {
        self.inner.call("terminate", &[]).as_::<Undefined>()
    }
}
