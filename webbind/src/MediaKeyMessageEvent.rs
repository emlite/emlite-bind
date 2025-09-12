use super::*;

/// The MediaKeyMessageEvent class.
/// [`MediaKeyMessageEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeyMessageEvent {
    inner: Event,
}

impl FromVal for MediaKeyMessageEvent {
    fn from_val(v: &Any) -> Self {
        MediaKeyMessageEvent {
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

impl core::ops::Deref for MediaKeyMessageEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaKeyMessageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaKeyMessageEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaKeyMessageEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaKeyMessageEvent> for Any {
    fn from(s: MediaKeyMessageEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaKeyMessageEvent> for Any {
    fn from(s: &MediaKeyMessageEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaKeyMessageEvent);

impl MediaKeyMessageEvent {
    /// Getter of the `messageType` attribute.
    /// [`MediaKeyMessageEvent.messageType`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/messageType)
    pub fn message_type(&self) -> MediaKeyMessageType {
        self.inner.get("messageType").as_::<MediaKeyMessageType>()
    }
}
impl MediaKeyMessageEvent {
    /// Getter of the `message` attribute.
    /// [`MediaKeyMessageEvent.message`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/message)
    pub fn message(&self) -> ArrayBuffer {
        self.inner.get("message").as_::<ArrayBuffer>()
    }
}

impl MediaKeyMessageEvent {
    /// The `new MediaKeyMessageEvent(..)` constructor, creating a new MediaKeyMessageEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &MediaKeyMessageEventInit,
    ) -> MediaKeyMessageEvent {
        Self {
            inner: Any::global("MediaKeyMessageEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
