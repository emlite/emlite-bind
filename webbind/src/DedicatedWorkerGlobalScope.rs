use super::*;




/// The DedicatedWorkerGlobalScope class.
/// [`DedicatedWorkerGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DedicatedWorkerGlobalScope {
    inner: WorkerGlobalScope,
}

impl FromVal for DedicatedWorkerGlobalScope {
    fn from_val(v: &Any) -> Self {
        DedicatedWorkerGlobalScope { inner: WorkerGlobalScope::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DedicatedWorkerGlobalScope {
    type Target = WorkerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DedicatedWorkerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DedicatedWorkerGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DedicatedWorkerGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DedicatedWorkerGlobalScope> for Any {
    fn from(s: DedicatedWorkerGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DedicatedWorkerGlobalScope> for Any {
    fn from(s: &DedicatedWorkerGlobalScope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DedicatedWorkerGlobalScope);


impl DedicatedWorkerGlobalScope {
    /// Getter of the `name` attribute.
    /// [`DedicatedWorkerGlobalScope.name`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

}
impl DedicatedWorkerGlobalScope {
    /// The postMessage method.
    /// [`DedicatedWorkerGlobalScope.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/postMessage)
    pub fn post_message0(&self, message: &Any) -> Undefined {
        self.inner.call("postMessage", &[message.into(), ]).as_::<Undefined>()
    }
    /// The postMessage method.
    /// [`DedicatedWorkerGlobalScope.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/postMessage)
    pub fn post_message1(&self, message: &Any, options: &StructuredSerializeOptions) -> Undefined {
        self.inner.call("postMessage", &[message.into(), options.into(), ]).as_::<Undefined>()
    }
}
impl DedicatedWorkerGlobalScope {
    /// The close method.
    /// [`DedicatedWorkerGlobalScope.close`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/close)
    pub fn close(&self, ) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl DedicatedWorkerGlobalScope {
    /// Getter of the `onrtctransform` attribute.
    /// [`DedicatedWorkerGlobalScope.onrtctransform`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onrtctransform)
    pub fn onrtctransform(&self) -> Any {
        self.inner.get("onrtctransform").as_::<Any>()
    }

    /// Setter of the `onrtctransform` attribute.
    /// [`DedicatedWorkerGlobalScope.onrtctransform`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onrtctransform)
    pub fn set_onrtctransform(&mut self, value: &Any) {
        self.inner.set("onrtctransform", value);
    }
}
impl DedicatedWorkerGlobalScope {
    /// The requestAnimationFrame method.
    /// [`DedicatedWorkerGlobalScope.requestAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/requestAnimationFrame)
    pub fn request_animation_frame(&self, callback: &Function) -> u32 {
        self.inner.call("requestAnimationFrame", &[callback.into(), ]).as_::<u32>()
    }
}
impl DedicatedWorkerGlobalScope {
    /// The cancelAnimationFrame method.
    /// [`DedicatedWorkerGlobalScope.cancelAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/cancelAnimationFrame)
    pub fn cancel_animation_frame(&self, handle: u32) -> Undefined {
        self.inner.call("cancelAnimationFrame", &[handle.into(), ]).as_::<Undefined>()
    }
}
impl DedicatedWorkerGlobalScope {
    /// Getter of the `onmessage` attribute.
    /// [`DedicatedWorkerGlobalScope.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`DedicatedWorkerGlobalScope.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl DedicatedWorkerGlobalScope {
    /// Getter of the `onmessageerror` attribute.
    /// [`DedicatedWorkerGlobalScope.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessageerror)
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    /// Setter of the `onmessageerror` attribute.
    /// [`DedicatedWorkerGlobalScope.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessageerror)
    pub fn set_onmessageerror(&mut self, value: &Any) {
        self.inner.set("onmessageerror", value);
    }
}
