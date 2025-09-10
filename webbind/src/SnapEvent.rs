use super::*;

/// The SnapEvent class.
/// [`SnapEvent`](https://developer.mozilla.org/en-US/docs/Web/API/SnapEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SnapEvent {
    inner: Event,
}

impl FromVal for SnapEvent {
    fn from_val(v: &Any) -> Self {
        SnapEvent {
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

impl core::ops::Deref for SnapEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SnapEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SnapEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SnapEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SnapEvent> for Any {
    fn from(s: SnapEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SnapEvent> for Any {
    fn from(s: &SnapEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SnapEvent);

impl SnapEvent {
    /// The `new SnapEvent(..)` constructor, creating a new SnapEvent instance
    pub fn new0(type_: &JsString) -> SnapEvent {
        Self {
            inner: Any::global("SnapEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    /// The `new SnapEvent(..)` constructor, creating a new SnapEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &SnapEventInit) -> SnapEvent {
        Self {
            inner: Any::global("SnapEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl SnapEvent {
    /// Getter of the `snapTargetBlock` attribute.
    /// [`SnapEvent.snapTargetBlock`](https://developer.mozilla.org/en-US/docs/Web/API/SnapEvent/snapTargetBlock)
    pub fn snap_target_block(&self) -> Node {
        self.inner.get("snapTargetBlock").as_::<Node>()
    }
}
impl SnapEvent {
    /// Getter of the `snapTargetInline` attribute.
    /// [`SnapEvent.snapTargetInline`](https://developer.mozilla.org/en-US/docs/Web/API/SnapEvent/snapTargetInline)
    pub fn snap_target_inline(&self) -> Node {
        self.inner.get("snapTargetInline").as_::<Node>()
    }
}
