use super::*;

/// The ClipboardEvent class.
/// [`ClipboardEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ClipboardEvent {
    inner: Event,
}

impl FromVal for ClipboardEvent {
    fn from_val(v: &Any) -> Self {
        ClipboardEvent {
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

impl core::ops::Deref for ClipboardEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ClipboardEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ClipboardEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ClipboardEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ClipboardEvent> for Any {
    fn from(s: ClipboardEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ClipboardEvent> for Any {
    fn from(s: &ClipboardEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ClipboardEvent);

impl ClipboardEvent {
    /// Getter of the `clipboardData` attribute.
    /// [`ClipboardEvent.clipboardData`](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent/clipboardData)
    pub fn clipboard_data(&self) -> DataTransfer {
        self.inner.get("clipboardData").as_::<DataTransfer>()
    }
}

impl ClipboardEvent {
    /// The `new ClipboardEvent(..)` constructor, creating a new ClipboardEvent instance
    pub fn new0(type_: &JsString) -> ClipboardEvent {
        Self {
            inner: Any::global("ClipboardEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new ClipboardEvent(..)` constructor, creating a new ClipboardEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &ClipboardEventInit) -> ClipboardEvent {
        Self {
            inner: Any::global("ClipboardEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
