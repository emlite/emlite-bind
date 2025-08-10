use super::*;

/// The SyncEvent class.
/// [`SyncEvent`](https://developer.mozilla.org/en-US/docs/Web/API/SyncEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SyncEvent {
    inner: ExtendableEvent,
}

impl FromVal for SyncEvent {
    fn from_val(v: &Any) -> Self {
        SyncEvent {
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

impl core::ops::Deref for SyncEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SyncEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SyncEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SyncEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SyncEvent> for Any {
    fn from(s: SyncEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SyncEvent> for Any {
    fn from(s: &SyncEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SyncEvent);

impl SyncEvent {
    /// The `new SyncEvent(..)` constructor, creating a new SyncEvent instance
    pub fn new(type_: &JsString, init: &SyncEventInit) -> SyncEvent {
        Self {
            inner: Any::global("SyncEvent")
                .new(&[type_.into(), init.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl SyncEvent {
    /// Getter of the `tag` attribute.
    /// [`SyncEvent.tag`](https://developer.mozilla.org/en-US/docs/Web/API/SyncEvent/tag)
    pub fn tag(&self) -> JsString {
        self.inner.get("tag").as_::<JsString>()
    }
}
impl SyncEvent {
    /// Getter of the `lastChance` attribute.
    /// [`SyncEvent.lastChance`](https://developer.mozilla.org/en-US/docs/Web/API/SyncEvent/lastChance)
    pub fn last_chance(&self) -> bool {
        self.inner.get("lastChance").as_::<bool>()
    }
}
