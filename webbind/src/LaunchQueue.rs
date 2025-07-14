use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LaunchQueue {
    inner: emlite::Val,
}
impl FromVal for LaunchQueue {
    fn from_val(v: &emlite::Val) -> Self {
        LaunchQueue {
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
impl core::ops::Deref for LaunchQueue {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LaunchQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LaunchQueue> for emlite::Val {
    fn from(s: LaunchQueue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LaunchQueue {
    pub fn set_consumer(&self, consumer: jsbind::Function) -> jsbind::Undefined {
        self.inner
            .call("setConsumer", &[consumer.into()])
            .as_::<jsbind::Undefined>()
    }
}
