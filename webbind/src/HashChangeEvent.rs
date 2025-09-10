use super::*;

/// The HashChangeEvent class.
/// [`HashChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HashChangeEvent {
    inner: Event,
}

impl FromVal for HashChangeEvent {
    fn from_val(v: &Any) -> Self {
        HashChangeEvent {
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

impl core::ops::Deref for HashChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HashChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HashChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HashChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HashChangeEvent> for Any {
    fn from(s: HashChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HashChangeEvent> for Any {
    fn from(s: &HashChangeEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HashChangeEvent);

impl HashChangeEvent {
    /// The `new HashChangeEvent(..)` constructor, creating a new HashChangeEvent instance
    pub fn new0(type_: &JsString) -> HashChangeEvent {
        Self {
            inner: Any::global("HashChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new HashChangeEvent(..)` constructor, creating a new HashChangeEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &HashChangeEventInit) -> HashChangeEvent {
        Self {
            inner: Any::global("HashChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl HashChangeEvent {
    /// Getter of the `oldURL` attribute.
    /// [`HashChangeEvent.oldURL`](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/oldURL)
    pub fn old_url(&self) -> JsString {
        self.inner.get("oldURL").as_::<JsString>()
    }
}
impl HashChangeEvent {
    /// Getter of the `newURL` attribute.
    /// [`HashChangeEvent.newURL`](https://developer.mozilla.org/en-US/docs/Web/API/HashChangeEvent/newURL)
    pub fn new_url(&self) -> JsString {
        self.inner.get("newURL").as_::<JsString>()
    }
}
