use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for WritableStreamDefaultController {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WritableStreamDefaultController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WritableStreamDefaultController {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WritableStreamDefaultController {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WritableStreamDefaultController> for emlite::Val {
    fn from(s: WritableStreamDefaultController) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WritableStreamDefaultController);

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
