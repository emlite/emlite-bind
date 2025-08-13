use super::*;




/// The SubmitEvent class.
/// [`SubmitEvent`](https://developer.mozilla.org/en-US/docs/Web/API/SubmitEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SubmitEvent {
    inner: Event,
}

impl FromVal for SubmitEvent {
    fn from_val(v: &Any) -> Self {
        SubmitEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for SubmitEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SubmitEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SubmitEvent> for Any {
    fn from(s: SubmitEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SubmitEvent> for Any {
    fn from(s: &SubmitEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SubmitEvent);



impl SubmitEvent {
    /// The `new SubmitEvent(..)` constructor, creating a new SubmitEvent instance
    pub fn new0(type_: &JsString) -> SubmitEvent {
        Self {
            inner: Any::global("SubmitEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    /// The `new SubmitEvent(..)` constructor, creating a new SubmitEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &SubmitEventInit) -> SubmitEvent {
        Self {
            inner: Any::global("SubmitEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl SubmitEvent {
    /// Getter of the `submitter` attribute.
    /// [`SubmitEvent.submitter`](https://developer.mozilla.org/en-US/docs/Web/API/SubmitEvent/submitter)
    pub fn submitter(&self) -> HTMLElement {
        self.inner.get("submitter").as_::<HTMLElement>()
    }

}
