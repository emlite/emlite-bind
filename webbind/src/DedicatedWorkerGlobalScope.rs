use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for DedicatedWorkerGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DedicatedWorkerGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DedicatedWorkerGlobalScope> for emlite::Val {
    fn from(s: DedicatedWorkerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&DedicatedWorkerGlobalScope> for emlite::Val {
    fn from(s: &DedicatedWorkerGlobalScope) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DedicatedWorkerGlobalScope);

impl DedicatedWorkerGlobalScope {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn post_message0(&self, message: &Any) -> Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<Undefined>()
    }

    pub fn post_message1(&self, message: &Any, options: &StructuredSerializeOptions) -> Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn onrtctransform(&self) -> Any {
        self.inner.get("onrtctransform").as_::<Any>()
    }

    pub fn set_onrtctransform(&mut self, value: &Any) {
        self.inner.set("onrtctransform", value);
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn request_animation_frame(&self, callback: &Function) -> u32 {
        self.inner
            .call("requestAnimationFrame", &[callback.into()])
            .as_::<u32>()
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn cancel_animation_frame(&self, handle: u32) -> Undefined {
        self.inner
            .call("cancelAnimationFrame", &[handle.into()])
            .as_::<Undefined>()
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl DedicatedWorkerGlobalScope {
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    pub fn set_onmessageerror(&mut self, value: &Any) {
        self.inner.set("onmessageerror", value);
    }
}
