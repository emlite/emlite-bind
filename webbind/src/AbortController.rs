use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AbortController {
    inner: emlite::Val,
}
impl FromVal for AbortController {
    fn from_val(v: &emlite::Val) -> Self {
        AbortController {
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
impl core::ops::Deref for AbortController {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AbortController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AbortController> for emlite::Val {
    fn from(s: AbortController) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AbortController {
    pub fn new() -> AbortController {
        Self {
            inner: emlite::Val::global("AbortController")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl AbortController {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }
}
impl AbortController {
    pub fn abort0(&self) -> jsbind::Undefined {
        self.inner.call("abort", &[]).as_::<jsbind::Undefined>()
    }

    pub fn abort1(&self, reason: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("abort", &[reason.into()])
            .as_::<jsbind::Undefined>()
    }
}
