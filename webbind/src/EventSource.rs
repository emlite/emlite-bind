use super::*;

/// The EventSource class.
/// [`EventSource`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EventSource {
    inner: EventTarget,
}
impl FromVal for EventSource {
    fn from_val(v: &Any) -> Self {
        EventSource {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for EventSource {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EventSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for EventSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for EventSource {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<EventSource> for Any {
    fn from(s: EventSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&EventSource> for Any {
    fn from(s: &EventSource) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(EventSource);

impl EventSource {
    /// The `new EventSource(..)` constructor, creating a new EventSource instance
    pub fn new0(url: &JsString) -> EventSource {
        Self {
            inner: Any::global("EventSource")
                .new(&[url.into()])
                .as_::<EventTarget>(),
        }
    }

    /// The `new EventSource(..)` constructor, creating a new EventSource instance
    pub fn new1(url: &JsString, event_source_init_dict: &EventSourceInit) -> EventSource {
        Self {
            inner: Any::global("EventSource")
                .new(&[url.into(), event_source_init_dict.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl EventSource {
    /// Getter of the `url` attribute.
    /// [`EventSource.url`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}
impl EventSource {
    /// Getter of the `withCredentials` attribute.
    /// [`EventSource.withCredentials`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/withCredentials)
    pub fn with_credentials(&self) -> bool {
        self.inner.get("withCredentials").as_::<bool>()
    }
}
impl EventSource {
    /// Getter of the `readyState` attribute.
    /// [`EventSource.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/readyState)
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl EventSource {
    /// Getter of the `onopen` attribute.
    /// [`EventSource.onopen`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)
    pub fn onopen(&self) -> Any {
        self.inner.get("onopen").as_::<Any>()
    }

    /// Setter of the `onopen` attribute.
    /// [`EventSource.onopen`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)
    pub fn set_onopen(&mut self, value: &Any) {
        self.inner.set("onopen", value);
    }
}
impl EventSource {
    /// Getter of the `onmessage` attribute.
    /// [`EventSource.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`EventSource.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl EventSource {
    /// Getter of the `onerror` attribute.
    /// [`EventSource.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`EventSource.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl EventSource {
    /// The close method.
    /// [`EventSource.close`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
