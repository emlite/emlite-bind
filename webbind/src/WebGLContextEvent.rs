use super::*;

/// The WebGLContextEvent class.
/// [`WebGLContextEvent`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLContextEvent {
    inner: Event,
}
impl FromVal for WebGLContextEvent {
    fn from_val(v: &Any) -> Self {
        WebGLContextEvent {
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
impl core::ops::Deref for WebGLContextEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLContextEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebGLContextEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebGLContextEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebGLContextEvent> for Any {
    fn from(s: WebGLContextEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebGLContextEvent> for Any {
    fn from(s: &WebGLContextEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebGLContextEvent);

impl WebGLContextEvent {
    /// The `new WebGLContextEvent(..)` constructor, creating a new WebGLContextEvent instance
    pub fn new0(type_: &str) -> WebGLContextEvent {
        Self {
            inner: Any::global("WebGLContextEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new WebGLContextEvent(..)` constructor, creating a new WebGLContextEvent instance
    pub fn new1(type_: &str, event_init: &Any) -> WebGLContextEvent {
        Self {
            inner: Any::global("WebGLContextEvent")
                .new(&[type_.into(), event_init.into()])
                .as_::<Event>(),
        }
    }
}
impl WebGLContextEvent {
    /// Getter of the `statusMessage` attribute.
    /// [`WebGLContextEvent.statusMessage`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLContextEvent/statusMessage)
    pub fn status_message(&self) -> String {
        self.inner.get("statusMessage").as_::<String>()
    }
}
