use super::*;

/// The BufferedChangeEvent class.
/// [`BufferedChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/BufferedChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BufferedChangeEvent {
    inner: Event,
}
impl FromVal for BufferedChangeEvent {
    fn from_val(v: &Any) -> Self {
        BufferedChangeEvent {
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
impl core::ops::Deref for BufferedChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BufferedChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BufferedChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BufferedChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BufferedChangeEvent> for Any {
    fn from(s: BufferedChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BufferedChangeEvent> for Any {
    fn from(s: &BufferedChangeEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BufferedChangeEvent);

impl BufferedChangeEvent {
    /// The `new BufferedChangeEvent(..)` constructor, creating a new BufferedChangeEvent instance
    pub fn new0(type_: &DOMString) -> BufferedChangeEvent {
        Self {
            inner: Any::global("BufferedChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new BufferedChangeEvent(..)` constructor, creating a new BufferedChangeEvent instance
    pub fn new1(type_: &DOMString, event_init_dict: &Any) -> BufferedChangeEvent {
        Self {
            inner: Any::global("BufferedChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl BufferedChangeEvent {
    /// Getter of the `addedRanges` attribute.
    /// [`BufferedChangeEvent.addedRanges`](https://developer.mozilla.org/en-US/docs/Web/API/BufferedChangeEvent/addedRanges)
    pub fn added_ranges(&self) -> TimeRanges {
        self.inner.get("addedRanges").as_::<TimeRanges>()
    }
}
impl BufferedChangeEvent {
    /// Getter of the `removedRanges` attribute.
    /// [`BufferedChangeEvent.removedRanges`](https://developer.mozilla.org/en-US/docs/Web/API/BufferedChangeEvent/removedRanges)
    pub fn removed_ranges(&self) -> TimeRanges {
        self.inner.get("removedRanges").as_::<TimeRanges>()
    }
}
