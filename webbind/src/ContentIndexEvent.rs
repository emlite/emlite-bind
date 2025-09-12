use super::*;

/// The ContentIndexEvent class.
/// [`ContentIndexEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ContentIndexEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContentIndexEvent {
    inner: ExtendableEvent,
}

impl FromVal for ContentIndexEvent {
    fn from_val(v: &Any) -> Self {
        ContentIndexEvent {
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

impl core::ops::Deref for ContentIndexEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ContentIndexEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ContentIndexEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ContentIndexEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ContentIndexEvent> for Any {
    fn from(s: ContentIndexEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ContentIndexEvent> for Any {
    fn from(s: &ContentIndexEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ContentIndexEvent);

impl ContentIndexEvent {
    /// Getter of the `id` attribute.
    /// [`ContentIndexEvent.id`](https://developer.mozilla.org/en-US/docs/Web/API/ContentIndexEvent/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}

impl ContentIndexEvent {
    /// The `new ContentIndexEvent(..)` constructor, creating a new ContentIndexEvent instance
    pub fn new(type_: &JsString, init: &ContentIndexEventInit) -> ContentIndexEvent {
        Self {
            inner: Any::global("ContentIndexEvent")
                .new(&[type_.into(), init.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
