use super::*;

/// The PeriodicSyncEvent class.
/// [`PeriodicSyncEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PeriodicSyncEvent {
    inner: ExtendableEvent,
}
impl FromVal for PeriodicSyncEvent {
    fn from_val(v: &Any) -> Self {
        PeriodicSyncEvent {
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
impl core::ops::Deref for PeriodicSyncEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PeriodicSyncEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PeriodicSyncEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PeriodicSyncEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PeriodicSyncEvent> for Any {
    fn from(s: PeriodicSyncEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PeriodicSyncEvent> for Any {
    fn from(s: &PeriodicSyncEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PeriodicSyncEvent);

impl PeriodicSyncEvent {
    /// The `new PeriodicSyncEvent(..)` constructor, creating a new PeriodicSyncEvent instance
    pub fn new(type_: &JsString, init: &PeriodicSyncEventInit) -> PeriodicSyncEvent {
        Self {
            inner: Any::global("PeriodicSyncEvent")
                .new(&[type_.into(), init.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl PeriodicSyncEvent {
    /// Getter of the `tag` attribute.
    /// [`PeriodicSyncEvent.tag`](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicSyncEvent/tag)
    pub fn tag(&self) -> JsString {
        self.inner.get("tag").as_::<JsString>()
    }
}
