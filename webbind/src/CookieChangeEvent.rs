use super::*;

/// The CookieChangeEvent class.
/// [`CookieChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CookieChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieChangeEvent {
    inner: Event,
}

impl FromVal for CookieChangeEvent {
    fn from_val(v: &Any) -> Self {
        CookieChangeEvent {
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

impl core::ops::Deref for CookieChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CookieChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CookieChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CookieChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CookieChangeEvent> for Any {
    fn from(s: CookieChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CookieChangeEvent> for Any {
    fn from(s: &CookieChangeEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CookieChangeEvent);

impl CookieChangeEvent {
    /// Getter of the `changed` attribute.
    /// [`CookieChangeEvent.changed`](https://developer.mozilla.org/en-US/docs/Web/API/CookieChangeEvent/changed)
    pub fn changed(&self) -> TypedArray<CookieListItem> {
        self.inner
            .get("changed")
            .as_::<TypedArray<CookieListItem>>()
    }
}
impl CookieChangeEvent {
    /// Getter of the `deleted` attribute.
    /// [`CookieChangeEvent.deleted`](https://developer.mozilla.org/en-US/docs/Web/API/CookieChangeEvent/deleted)
    pub fn deleted(&self) -> TypedArray<CookieListItem> {
        self.inner
            .get("deleted")
            .as_::<TypedArray<CookieListItem>>()
    }
}

impl CookieChangeEvent {
    /// The `new CookieChangeEvent(..)` constructor, creating a new CookieChangeEvent instance
    pub fn new0(type_: &JsString) -> CookieChangeEvent {
        Self {
            inner: Any::global("CookieChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new CookieChangeEvent(..)` constructor, creating a new CookieChangeEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &CookieChangeEventInit) -> CookieChangeEvent {
        Self {
            inner: Any::global("CookieChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
