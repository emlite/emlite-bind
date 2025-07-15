use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for Subscriber {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Subscriber {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(Subscriber);

impl Subscriber {
    pub fn next(&self, value: Any) -> Undefined {
        self.inner.call("next", &[value.into()]).as_::<Undefined>()
    }
}
impl Subscriber {
    pub fn error(&self, error: Any) -> Undefined {
        self.inner.call("error", &[error.into()]).as_::<Undefined>()
    }
}
impl Subscriber {
    pub fn complete(&self) -> Undefined {
        self.inner.call("complete", &[]).as_::<Undefined>()
    }
}
impl Subscriber {
    pub fn add_teardown(&self, teardown: Any) -> Undefined {
        self.inner
            .call("addTeardown", &[teardown.into()])
            .as_::<Undefined>()
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
