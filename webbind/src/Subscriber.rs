use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Subscriber {
    inner: emlite::Val,
}
impl FromVal for Subscriber {
    fn from_val(v: &emlite::Val) -> Self {
        Subscriber {
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
impl core::ops::Deref for Subscriber {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Subscriber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Subscriber> for emlite::Val {
    fn from(s: Subscriber) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Subscriber {
    pub fn next(&self, value: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("next", &[value.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Subscriber {
    pub fn error(&self, error: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("error", &[error.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Subscriber {
    pub fn complete(&self) -> jsbind::Undefined {
        self.inner.call("complete", &[]).as_::<jsbind::Undefined>()
    }
}
impl Subscriber {
    pub fn add_teardown(&self, teardown: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("addTeardown", &[teardown.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Subscriber {
    pub fn active(&self) -> bool {
        self.inner.get("active").as_::<bool>()
    }
}
impl Subscriber {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }
}
