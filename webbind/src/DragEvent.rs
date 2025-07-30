use super::*;

/// The DragEvent class.
/// [`DragEvent`](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DragEvent {
    inner: MouseEvent,
}
impl FromVal for DragEvent {
    fn from_val(v: &Any) -> Self {
        DragEvent {
            inner: MouseEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DragEvent {
    type Target = MouseEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DragEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DragEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DragEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DragEvent> for Any {
    fn from(s: DragEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DragEvent> for Any {
    fn from(s: &DragEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DragEvent);

impl DragEvent {
    /// The `new DragEvent(..)` constructor, creating a new DragEvent instance
    pub fn new0(type_: &JsString) -> DragEvent {
        Self {
            inner: Any::global("DragEvent")
                .new(&[type_.into()])
                .as_::<MouseEvent>(),
        }
    }

    /// The `new DragEvent(..)` constructor, creating a new DragEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &Any) -> DragEvent {
        Self {
            inner: Any::global("DragEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<MouseEvent>(),
        }
    }
}
impl DragEvent {
    /// Getter of the `dataTransfer` attribute.
    /// [`DragEvent.dataTransfer`](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/dataTransfer)
    pub fn data_transfer(&self) -> DataTransfer {
        self.inner.get("dataTransfer").as_::<DataTransfer>()
    }
}
