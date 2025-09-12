use super::*;

/// The ValueEvent class.
/// [`ValueEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ValueEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ValueEvent {
    inner: Event,
}

impl FromVal for ValueEvent {
    fn from_val(v: &Any) -> Self {
        ValueEvent {
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

impl core::ops::Deref for ValueEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ValueEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ValueEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ValueEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ValueEvent> for Any {
    fn from(s: ValueEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ValueEvent> for Any {
    fn from(s: &ValueEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ValueEvent);

impl ValueEvent {
    /// Getter of the `value` attribute.
    /// [`ValueEvent.value`](https://developer.mozilla.org/en-US/docs/Web/API/ValueEvent/value)
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }
}

impl ValueEvent {
    /// The `new ValueEvent(..)` constructor, creating a new ValueEvent instance
    pub fn new(type_: &JsString) -> ValueEvent {
        Self {
            inner: Any::global("ValueEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }
}

impl ValueEvent {
    /// The `new ValueEvent(..)` constructor, creating a new ValueEvent instance
    pub fn new_with_init_dict(type_: &JsString, init_dict: &ValueEventInit) -> ValueEvent {
        Self {
            inner: Any::global("ValueEvent")
                .new(&[type_.into(), init_dict.into()])
                .as_::<Event>(),
        }
    }
}
