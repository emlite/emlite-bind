use super::*;

#[derive(Clone, Debug)]
pub struct CreateMonitor {
    inner: EventTarget,
}
impl FromVal for CreateMonitor {
    fn from_val(v: &emlite::Val) -> Self {
        CreateMonitor {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CreateMonitor {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CreateMonitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CreateMonitor> for emlite::Val {
    fn from(s: CreateMonitor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CreateMonitor {
    pub fn ondownloadprogress(&self) -> jsbind::Any {
        self.inner.get("ondownloadprogress").as_::<jsbind::Any>()
    }

    pub fn set_ondownloadprogress(&mut self, value: jsbind::Any) {
        self.inner.set("ondownloadprogress", value);
    }
}
