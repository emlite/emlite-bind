use super::*;

#[derive(Clone, Debug)]
pub struct DedicatedWorkerGlobalScope {
    inner: WorkerGlobalScope,
}
impl FromVal for DedicatedWorkerGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        DedicatedWorkerGlobalScope {
            inner: WorkerGlobalScope::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for DedicatedWorkerGlobalScope {
    type Target = WorkerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DedicatedWorkerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DedicatedWorkerGlobalScope> for emlite::Val {
    fn from(s: DedicatedWorkerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DedicatedWorkerGlobalScope {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn post_message0(&self, message: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn post_message1(
        &self,
        message: jsbind::Any,
        options: StructuredSerializeOptions,
    ) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn onrtctransform(&self) -> jsbind::Any {
        self.inner.get("onrtctransform").as_::<jsbind::Any>()
    }

    pub fn set_onrtctransform(&mut self, value: jsbind::Any) {
        self.inner.set("onrtctransform", value);
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn request_animation_frame(&self, callback: jsbind::Function) -> u32 {
        self.inner
            .call("requestAnimationFrame", &[callback.into()])
            .as_::<u32>()
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn cancel_animation_frame(&self, handle: u32) -> jsbind::Undefined {
        self.inner
            .call("cancelAnimationFrame", &[handle.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn onmessageerror(&self) -> jsbind::Any {
        self.inner.get("onmessageerror").as_::<jsbind::Any>()
    }

    pub fn set_onmessageerror(&mut self, value: jsbind::Any) {
        self.inner.set("onmessageerror", value);
    }
}
