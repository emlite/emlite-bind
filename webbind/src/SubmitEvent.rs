use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for SubmitEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SubmitEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SubmitEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SubmitEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SubmitEvent> for emlite::Val {
    fn from(s: SubmitEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SubmitEvent> for emlite::Val {
    fn from(s: &SubmitEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SubmitEvent);

impl SubmitEvent {
    pub fn new0(type_: DOMString) -> SubmitEvent {
        Self {
            inner: emlite::Val::global("SubmitEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> SubmitEvent {
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
