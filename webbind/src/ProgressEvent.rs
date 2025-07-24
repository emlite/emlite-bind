use super::*;

/// The ProgressEvent class.
/// [`ProgressEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProgressEvent {
    inner: Event,
}
impl FromVal for ProgressEvent {
    fn from_val(v: &Any) -> Self {
        ProgressEvent {
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
impl core::ops::Deref for ProgressEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ProgressEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ProgressEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ProgressEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ProgressEvent> for Any {
    fn from(s: ProgressEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ProgressEvent> for Any {
    fn from(s: &ProgressEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ProgressEvent);

impl ProgressEvent {
    /// The `new ProgressEvent(..)` constructor, creating a new ProgressEvent instance
    pub fn new0(type_: &DOMString) -> ProgressEvent {
        Self {
            inner: Any::global("ProgressEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new ProgressEvent(..)` constructor, creating a new ProgressEvent instance
    pub fn new1(type_: &DOMString, event_init_dict: &Any) -> ProgressEvent {
        Self {
            inner: Any::global("ProgressEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ProgressEvent {
    /// Getter of the `lengthComputable` attribute.
    /// [`ProgressEvent.lengthComputable`](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/lengthComputable)
    pub fn length_computable(&self) -> bool {
        self.inner.get("lengthComputable").as_::<bool>()
    }
}
impl ProgressEvent {
    /// Getter of the `loaded` attribute.
    /// [`ProgressEvent.loaded`](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/loaded)
    pub fn loaded(&self) -> f64 {
        self.inner.get("loaded").as_::<f64>()
    }
}
impl ProgressEvent {
    /// Getter of the `total` attribute.
    /// [`ProgressEvent.total`](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent/total)
    pub fn total(&self) -> f64 {
        self.inner.get("total").as_::<f64>()
    }
}
