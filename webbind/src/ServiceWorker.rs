use super::*;




/// The ServiceWorker class.
/// [`ServiceWorker`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ServiceWorker {
    inner: EventTarget,
}

impl FromVal for ServiceWorker {
    fn from_val(v: &Any) -> Self {
        ServiceWorker { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ServiceWorker {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ServiceWorker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ServiceWorker {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ServiceWorker {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ServiceWorker> for Any {
    fn from(s: ServiceWorker) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ServiceWorker> for Any {
    fn from(s: &ServiceWorker) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ServiceWorker);


impl ServiceWorker {
    /// Getter of the `scriptURL` attribute.
    /// [`ServiceWorker.scriptURL`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/scriptURL)
    pub fn script_url(&self) -> JsString {
        self.inner.get("scriptURL").as_::<JsString>()
    }

}
impl ServiceWorker {
    /// Getter of the `state` attribute.
    /// [`ServiceWorker.state`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/state)
    pub fn state(&self) -> ServiceWorkerState {
        self.inner.get("state").as_::<ServiceWorkerState>()
    }

}
impl ServiceWorker {
    /// The postMessage method.
    /// [`ServiceWorker.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/postMessage)
    pub fn post_message0(&self, message: &Any) -> Undefined {
        self.inner.call("postMessage", &[message.into(), ]).as_::<Undefined>()
    }
    /// The postMessage method.
    /// [`ServiceWorker.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/postMessage)
    pub fn post_message1(&self, message: &Any, options: &StructuredSerializeOptions) -> Undefined {
        self.inner.call("postMessage", &[message.into(), options.into(), ]).as_::<Undefined>()
    }
}
impl ServiceWorker {
    /// Getter of the `onstatechange` attribute.
    /// [`ServiceWorker.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onstatechange)
    pub fn onstatechange(&self) -> Any {
        self.inner.get("onstatechange").as_::<Any>()
    }

    /// Setter of the `onstatechange` attribute.
    /// [`ServiceWorker.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onstatechange)
    pub fn set_onstatechange(&mut self, value: &Any) {
        self.inner.set("onstatechange", value);
    }
}
impl ServiceWorker {
    /// Getter of the `onerror` attribute.
    /// [`ServiceWorker.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`ServiceWorker.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
