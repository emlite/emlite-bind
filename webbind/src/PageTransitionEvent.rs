use super::*;

#[derive(Clone, Debug)]
pub struct PageTransitionEvent {
    inner: Event,
}
impl FromVal for PageTransitionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PageTransitionEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PageTransitionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PageTransitionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PageTransitionEvent> for emlite::Val {
    fn from(s: PageTransitionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PageTransitionEvent {
    pub fn new0(type_: jsbind::DOMString) -> PageTransitionEvent {
        Self {
            inner: emlite::Val::global("PageTransitionEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> PageTransitionEvent {
        Self {
            inner: emlite::Val::global("PageTransitionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PageTransitionEvent {
    pub fn persisted(&self) -> bool {
        self.inner.get("persisted").as_::<bool>()
    }
}
