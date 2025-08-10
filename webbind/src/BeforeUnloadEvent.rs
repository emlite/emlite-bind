use super::*;

/// The BeforeUnloadEvent class.
/// [`BeforeUnloadEvent`](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BeforeUnloadEvent {
    inner: Event,
}

impl FromVal for BeforeUnloadEvent {
    fn from_val(v: &Any) -> Self {
        BeforeUnloadEvent {
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

impl core::ops::Deref for BeforeUnloadEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BeforeUnloadEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BeforeUnloadEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BeforeUnloadEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BeforeUnloadEvent> for Any {
    fn from(s: BeforeUnloadEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BeforeUnloadEvent> for Any {
    fn from(s: &BeforeUnloadEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BeforeUnloadEvent);

impl BeforeUnloadEvent {
    /// Getter of the `returnValue` attribute.
    /// [`BeforeUnloadEvent.returnValue`](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)
    pub fn return_value(&self) -> JsString {
        self.inner.get("returnValue").as_::<JsString>()
    }

    /// Setter of the `returnValue` attribute.
    /// [`BeforeUnloadEvent.returnValue`](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)
    pub fn set_return_value(&mut self, value: &JsString) {
        self.inner.set("returnValue", value);
    }
}
