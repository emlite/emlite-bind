use super::*;

#[derive(Clone, Debug)]
pub struct SubmitEvent {
    inner: Event,
}
impl FromVal for SubmitEvent {
    fn from_val(v: &emlite::Val) -> Self {
        SubmitEvent {
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
impl std::ops::Deref for SubmitEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SubmitEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SubmitEvent> for emlite::Val {
    fn from(s: SubmitEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SubmitEvent {
    pub fn new0(type_: jsbind::DOMString) -> SubmitEvent {
        Self {
            inner: emlite::Val::global("SubmitEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> SubmitEvent {
        Self {
            inner: emlite::Val::global("SubmitEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl SubmitEvent {
    pub fn submitter(&self) -> HTMLElement {
        self.inner.get("submitter").as_::<HTMLElement>()
    }
}
