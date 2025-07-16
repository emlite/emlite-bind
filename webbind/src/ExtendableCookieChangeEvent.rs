use super::*;

/// The ExtendableCookieChangeEvent class.
/// [`ExtendableCookieChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableCookieChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ExtendableCookieChangeEvent {
    inner: ExtendableEvent,
}
impl FromVal for ExtendableCookieChangeEvent {
    fn from_val(v: &Any) -> Self {
        ExtendableCookieChangeEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ExtendableCookieChangeEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ExtendableCookieChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ExtendableCookieChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ExtendableCookieChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ExtendableCookieChangeEvent> for Any {
    fn from(s: ExtendableCookieChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ExtendableCookieChangeEvent> for Any {
    fn from(s: &ExtendableCookieChangeEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ExtendableCookieChangeEvent);

impl ExtendableCookieChangeEvent {
    /// The `new ExtendableCookieChangeEvent(..)` constructor, creating a new ExtendableCookieChangeEvent instance
    pub fn new0(type_: &str) -> ExtendableCookieChangeEvent {
        Self {
            inner: Any::global("ExtendableCookieChangeEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }

    /// The `new ExtendableCookieChangeEvent(..)` constructor, creating a new ExtendableCookieChangeEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> ExtendableCookieChangeEvent {
        Self {
            inner: Any::global("ExtendableCookieChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl ExtendableCookieChangeEvent {
    /// Getter of the `changed` attribute.
    /// [`ExtendableCookieChangeEvent.changed`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableCookieChangeEvent/changed)
    pub fn changed(&self) -> FrozenArray<CookieListItem> {
        self.inner
            .get("changed")
            .as_::<FrozenArray<CookieListItem>>()
    }
}
impl ExtendableCookieChangeEvent {
    /// Getter of the `deleted` attribute.
    /// [`ExtendableCookieChangeEvent.deleted`](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableCookieChangeEvent/deleted)
    pub fn deleted(&self) -> FrozenArray<CookieListItem> {
        self.inner
            .get("deleted")
            .as_::<FrozenArray<CookieListItem>>()
    }
}
