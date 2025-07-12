use super::*;

#[derive(Clone, Debug)]
pub struct PresentationAvailability {
    inner: EventTarget,
}
impl FromVal for PresentationAvailability {
    fn from_val(v: &emlite::Val) -> Self {
        PresentationAvailability {
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
impl std::ops::Deref for PresentationAvailability {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PresentationAvailability {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PresentationAvailability> for emlite::Val {
    fn from(s: PresentationAvailability) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PresentationAvailability {
    pub fn value(&self) -> bool {
        self.inner.get("value").as_::<bool>()
    }
}
impl PresentationAvailability {
    pub fn onchange(&self) -> jsbind::Any {
        self.inner.get("onchange").as_::<jsbind::Any>()
    }

    pub fn set_onchange(&mut self, value: jsbind::Any) {
        self.inner.set("onchange", value);
    }
}
