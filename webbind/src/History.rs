use super::*;

/// The History class.
/// [`History`](https://developer.mozilla.org/en-US/docs/Web/API/History)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct History {
    inner: Any,
}
impl FromVal for History {
    fn from_val(v: &Any) -> Self {
        History {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for History {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for History {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for History {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for History {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<History> for Any {
    fn from(s: History) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&History> for Any {
    fn from(s: &History) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(History);

impl History {
    /// Getter of the `length` attribute.
    /// [`History.length`](https://developer.mozilla.org/en-US/docs/Web/API/History/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl History {
    /// Getter of the `scrollRestoration` attribute.
    /// [`History.scrollRestoration`](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)
    pub fn scroll_restoration(&self) -> ScrollRestoration {
        self.inner
            .get("scrollRestoration")
            .as_::<ScrollRestoration>()
    }

    /// Setter of the `scrollRestoration` attribute.
    /// [`History.scrollRestoration`](https://developer.mozilla.org/en-US/docs/Web/API/History/scrollRestoration)
    pub fn set_scroll_restoration(&mut self, value: &ScrollRestoration) {
        self.inner.set("scrollRestoration", value);
    }
}
impl History {
    /// Getter of the `state` attribute.
    /// [`History.state`](https://developer.mozilla.org/en-US/docs/Web/API/History/state)
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }
}
impl History {
    /// The go method.
    /// [`History.go`](https://developer.mozilla.org/en-US/docs/Web/API/History/go)
    pub fn go0(&self) -> Undefined {
        self.inner.call("go", &[]).as_::<Undefined>()
    }
    /// The go method.
    /// [`History.go`](https://developer.mozilla.org/en-US/docs/Web/API/History/go)
    pub fn go1(&self, delta: i32) -> Undefined {
        self.inner.call("go", &[delta.into()]).as_::<Undefined>()
    }
}
impl History {
    /// The back method.
    /// [`History.back`](https://developer.mozilla.org/en-US/docs/Web/API/History/back)
    pub fn back(&self) -> Undefined {
        self.inner.call("back", &[]).as_::<Undefined>()
    }
}
impl History {
    /// The forward method.
    /// [`History.forward`](https://developer.mozilla.org/en-US/docs/Web/API/History/forward)
    pub fn forward(&self) -> Undefined {
        self.inner.call("forward", &[]).as_::<Undefined>()
    }
}
impl History {
    /// The pushState method.
    /// [`History.pushState`](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)
    pub fn push_state0(&self, data: &Any, unused: &JsString) -> Undefined {
        self.inner
            .call("pushState", &[data.into(), unused.into()])
            .as_::<Undefined>()
    }
    /// The pushState method.
    /// [`History.pushState`](https://developer.mozilla.org/en-US/docs/Web/API/History/pushState)
    pub fn push_state1(&self, data: &Any, unused: &JsString, url: &JsString) -> Undefined {
        self.inner
            .call("pushState", &[data.into(), unused.into(), url.into()])
            .as_::<Undefined>()
    }
}
impl History {
    /// The replaceState method.
    /// [`History.replaceState`](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)
    pub fn replace_state0(&self, data: &Any, unused: &JsString) -> Undefined {
        self.inner
            .call("replaceState", &[data.into(), unused.into()])
            .as_::<Undefined>()
    }
    /// The replaceState method.
    /// [`History.replaceState`](https://developer.mozilla.org/en-US/docs/Web/API/History/replaceState)
    pub fn replace_state1(&self, data: &Any, unused: &JsString, url: &JsString) -> Undefined {
        self.inner
            .call("replaceState", &[data.into(), unused.into(), url.into()])
            .as_::<Undefined>()
    }
}
