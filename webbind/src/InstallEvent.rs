use super::*;

#[derive(Clone, Debug)]
pub struct InstallEvent {
    inner: ExtendableEvent,
}
impl FromVal for InstallEvent {
    fn from_val(v: &emlite::Val) -> Self {
        InstallEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for InstallEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for InstallEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<InstallEvent> for emlite::Val {
    fn from(s: InstallEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl InstallEvent {
    pub fn new0(type_: jsbind::DOMString) -> InstallEvent {
        Self {
            inner: emlite::Val::global("InstallEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> InstallEvent {
        Self {
            inner: emlite::Val::global("InstallEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl InstallEvent {
    pub fn add_routes(&self, rules: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("addRoutes", &[rules.into()])
            .as_::<jsbind::Promise>()
    }
}
