use super::*;

/// The IDBVersionChangeEvent class.
/// [`IDBVersionChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBVersionChangeEvent {
    inner: Event,
}

impl FromVal for IDBVersionChangeEvent {
    fn from_val(v: &Any) -> Self {
        IDBVersionChangeEvent {
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

impl core::ops::Deref for IDBVersionChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IDBVersionChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IDBVersionChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IDBVersionChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<IDBVersionChangeEvent> for Any {
    fn from(s: IDBVersionChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IDBVersionChangeEvent> for Any {
    fn from(s: &IDBVersionChangeEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(IDBVersionChangeEvent);

impl IDBVersionChangeEvent {
    /// Getter of the `oldVersion` attribute.
    /// [`IDBVersionChangeEvent.oldVersion`](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/oldVersion)
    pub fn old_version(&self) -> u64 {
        self.inner.get("oldVersion").as_::<u64>()
    }
}
impl IDBVersionChangeEvent {
    /// Getter of the `newVersion` attribute.
    /// [`IDBVersionChangeEvent.newVersion`](https://developer.mozilla.org/en-US/docs/Web/API/IDBVersionChangeEvent/newVersion)
    pub fn new_version(&self) -> u64 {
        self.inner.get("newVersion").as_::<u64>()
    }
}

impl IDBVersionChangeEvent {
    /// The `new IDBVersionChangeEvent(..)` constructor, creating a new IDBVersionChangeEvent instance
    pub fn new(type_: &JsString) -> IDBVersionChangeEvent {
        Self {
            inner: Any::global("IDBVersionChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }
}

impl IDBVersionChangeEvent {
    /// The `new IDBVersionChangeEvent(..)` constructor, creating a new IDBVersionChangeEvent instance
    pub fn new_with_event_init_dict(
        type_: &JsString,
        event_init_dict: &IDBVersionChangeEventInit,
    ) -> IDBVersionChangeEvent {
        Self {
            inner: Any::global("IDBVersionChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
