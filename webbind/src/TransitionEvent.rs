use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TransitionEvent {
    inner: Event,
}
impl FromVal for TransitionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        TransitionEvent {
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
impl core::ops::Deref for TransitionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TransitionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TransitionEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TransitionEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TransitionEvent> for emlite::Val {
    fn from(s: TransitionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TransitionEvent);

impl TransitionEvent {
    pub fn new0(type_: CSSOMString) -> TransitionEvent {
        Self {
            inner: emlite::Val::global("TransitionEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: CSSOMString, transition_event_init_dict: Any) -> TransitionEvent {
        Self {
            inner: emlite::Val::global("TransitionEvent")
                .new(&[type_.into(), transition_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl TransitionEvent {
    pub fn property_name(&self) -> CSSOMString {
        self.inner.get("propertyName").as_::<CSSOMString>()
    }
}
impl TransitionEvent {
    pub fn elapsed_time(&self) -> f64 {
        self.inner.get("elapsedTime").as_::<f64>()
    }
}
impl TransitionEvent {
    pub fn pseudo_element(&self) -> CSSOMString {
        self.inner.get("pseudoElement").as_::<CSSOMString>()
    }
}
