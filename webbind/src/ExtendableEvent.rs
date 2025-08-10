use super::*;

/// The ExtendableEvent class.
/// [`ExtendableEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ExtendableEvent {
    inner: Event,
}

impl FromVal for ExtendableEvent {
    fn from_val(v: &Any) -> Self {
        ExtendableEvent {
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

impl core::ops::Deref for ExtendableEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ExtendableEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ExtendableEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ExtendableEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ExtendableEvent> for Any {
    fn from(s: ExtendableEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ExtendableEvent> for Any {
    fn from(s: &ExtendableEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ExtendableEvent);

impl ExtendableEvent {
    /// The `new ExtendableEvent(..)` constructor, creating a new ExtendableEvent instance
    pub fn new0(type_: &JsString) -> ExtendableEvent {
        Self {
            inner: Any::global("ExtendableEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new ExtendableEvent(..)` constructor, creating a new ExtendableEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &ExtendableEventInit) -> ExtendableEvent {
        Self {
            inner: Any::global("ExtendableEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ExtendableEvent {
    /// The waitUntil method.
    /// [`ExtendableEvent.waitUntil`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/waitUntil)
    pub fn wait_until(&self, f: &Promise<Any>) -> Undefined {
        self.inner.call("waitUntil", &[f.into()]).as_::<Undefined>()
    }
}
