use super::*;

/// The FormDataEvent class.
/// [`FormDataEvent`](https://developer.mozilla.org/en-US/docs/Web/API/FormDataEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FormDataEvent {
    inner: Event,
}
impl FromVal for FormDataEvent {
    fn from_val(v: &Any) -> Self {
        FormDataEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FormDataEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FormDataEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FormDataEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FormDataEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FormDataEvent> for Any {
    fn from(s: FormDataEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FormDataEvent> for Any {
    fn from(s: &FormDataEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FormDataEvent);

impl FormDataEvent {
    /// The `new FormDataEvent(..)` constructor, creating a new FormDataEvent instance
    pub fn new(type_: &JsString, event_init_dict: &Any) -> FormDataEvent {
        Self {
            inner: Any::global("FormDataEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl FormDataEvent {
    /// Getter of the `formData` attribute.
    /// [`FormDataEvent.formData`](https://developer.mozilla.org/en-US/docs/Web/API/FormDataEvent/formData)
    pub fn form_data(&self) -> FormData {
        self.inner.get("formData").as_::<FormData>()
    }
}
