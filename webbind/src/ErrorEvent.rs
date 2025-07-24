use super::*;

/// The ErrorEvent class.
/// [`ErrorEvent`](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ErrorEvent {
    inner: Event,
}
impl FromVal for ErrorEvent {
    fn from_val(v: &Any) -> Self {
        ErrorEvent {
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
impl core::ops::Deref for ErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ErrorEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ErrorEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ErrorEvent> for Any {
    fn from(s: ErrorEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ErrorEvent> for Any {
    fn from(s: &ErrorEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ErrorEvent);

impl ErrorEvent {
    /// The `new ErrorEvent(..)` constructor, creating a new ErrorEvent instance
    pub fn new0(type_: &DOMString) -> ErrorEvent {
        Self {
            inner: Any::global("ErrorEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new ErrorEvent(..)` constructor, creating a new ErrorEvent instance
    pub fn new1(type_: &DOMString, event_init_dict: &Any) -> ErrorEvent {
        Self {
            inner: Any::global("ErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ErrorEvent {
    /// Getter of the `message` attribute.
    /// [`ErrorEvent.message`](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/message)
    pub fn message(&self) -> DOMString {
        self.inner.get("message").as_::<DOMString>()
    }
}
impl ErrorEvent {
    /// Getter of the `filename` attribute.
    /// [`ErrorEvent.filename`](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/filename)
    pub fn filename(&self) -> USVString {
        self.inner.get("filename").as_::<USVString>()
    }
}
impl ErrorEvent {
    /// Getter of the `lineno` attribute.
    /// [`ErrorEvent.lineno`](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/lineno)
    pub fn lineno(&self) -> u32 {
        self.inner.get("lineno").as_::<u32>()
    }
}
impl ErrorEvent {
    /// Getter of the `colno` attribute.
    /// [`ErrorEvent.colno`](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/colno)
    pub fn colno(&self) -> u32 {
        self.inner.get("colno").as_::<u32>()
    }
}
impl ErrorEvent {
    /// Getter of the `error` attribute.
    /// [`ErrorEvent.error`](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent/error)
    pub fn error(&self) -> Any {
        self.inner.get("error").as_::<Any>()
    }
}
