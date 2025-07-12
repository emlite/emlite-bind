use super::*;

#[derive(Clone, Debug)]
pub struct IdleDeadline {
    inner: emlite::Val,
}
impl FromVal for IdleDeadline {
    fn from_val(v: &emlite::Val) -> Self {
        IdleDeadline {
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
impl std::ops::Deref for IdleDeadline {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for IdleDeadline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IdleDeadline> for emlite::Val {
    fn from(s: IdleDeadline) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdleDeadline {
    pub fn time_remaining(&self) -> jsbind::Any {
        self.inner.call("timeRemaining", &[]).as_::<jsbind::Any>()
    }
}
impl IdleDeadline {
    pub fn did_timeout(&self) -> bool {
        self.inner.get("didTimeout").as_::<bool>()
    }
}
