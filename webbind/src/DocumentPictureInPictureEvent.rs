use super::*;

/// The DocumentPictureInPictureEvent class.
/// [`DocumentPictureInPictureEvent`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPictureEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentPictureInPictureEvent {
    inner: Event,
}
impl FromVal for DocumentPictureInPictureEvent {
    fn from_val(v: &Any) -> Self {
        DocumentPictureInPictureEvent {
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
impl core::ops::Deref for DocumentPictureInPictureEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DocumentPictureInPictureEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DocumentPictureInPictureEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DocumentPictureInPictureEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DocumentPictureInPictureEvent> for Any {
    fn from(s: DocumentPictureInPictureEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DocumentPictureInPictureEvent> for Any {
    fn from(s: &DocumentPictureInPictureEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DocumentPictureInPictureEvent);

impl DocumentPictureInPictureEvent {
    /// The `new DocumentPictureInPictureEvent(..)` constructor, creating a new DocumentPictureInPictureEvent instance
    pub fn new(type_: &str, event_init_dict: &Any) -> DocumentPictureInPictureEvent {
        Self {
            inner: Any::global("DocumentPictureInPictureEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl DocumentPictureInPictureEvent {
    /// Getter of the `window` attribute.
    /// [`DocumentPictureInPictureEvent.window`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPictureEvent/window)
    pub fn window(&self) -> Window {
        self.inner.get("window").as_::<Window>()
    }
}
