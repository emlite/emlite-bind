use super::*;

/// The Worker class.
/// [`Worker`](https://developer.mozilla.org/en-US/docs/Web/API/Worker)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Worker {
    inner: EventTarget,
}

impl FromVal for Worker {
    fn from_val(v: &Any) -> Self {
        Worker {
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

impl core::ops::Deref for Worker {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Worker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Worker {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Worker {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Worker> for Any {
    fn from(s: Worker) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Worker> for Any {
    fn from(s: &Worker) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Worker);

impl Worker {
    /// The `new Worker(..)` constructor, creating a new Worker instance
    pub fn new0(script_url: &Any) -> Worker {
        Self {
            inner: Any::global("Worker")
                .new(&[script_url.into()])
                .as_::<EventTarget>(),
        }
    }

    /// The `new Worker(..)` constructor, creating a new Worker instance
    pub fn new1(script_url: &Any, options: &WorkerOptions) -> Worker {
        Self {
            inner: Any::global("Worker")
                .new(&[script_url.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl Worker {
    /// The terminate method.
    /// [`Worker.terminate`](https://developer.mozilla.org/en-US/docs/Web/API/Worker/terminate)
    pub fn terminate(&self) -> Undefined {
        self.inner.call("terminate", &[]).as_::<Undefined>()
    }
}
impl Worker {
    /// The postMessage method.
    /// [`Worker.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)
    pub fn post_message0(&self, message: &Any) -> Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<Undefined>()
    }
    /// The postMessage method.
    /// [`Worker.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)
    pub fn post_message1(&self, message: &Any, options: &StructuredSerializeOptions) -> Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl Worker {
    /// Getter of the `onerror` attribute.
    /// [`Worker.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`Worker.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl Worker {
    /// Getter of the `onmessage` attribute.
    /// [`Worker.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`Worker.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl Worker {
    /// Getter of the `onmessageerror` attribute.
    /// [`Worker.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    /// Setter of the `onmessageerror` attribute.
    /// [`Worker.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)
    pub fn set_onmessageerror(&mut self, value: &Any) {
        self.inner.set("onmessageerror", value);
    }
}
