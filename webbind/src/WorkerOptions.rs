use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkerOptions {
    inner: Any,
}
impl FromVal for WorkerOptions {
    fn from_val(v: &Any) -> Self {
        WorkerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WorkerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WorkerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WorkerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WorkerOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WorkerOptions> for Any {
    fn from(s: WorkerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WorkerOptions> for Any {
    fn from(s: &WorkerOptions) -> Any {
        s.inner.clone()
    }
}

impl WorkerOptions {
    pub fn type_(&self) -> WorkerType {
        self.inner.get("type").as_::<WorkerType>()
    }

    pub fn set_type_(&mut self, value: &WorkerType) {
        self.inner.set("type", value);
    }
}
impl WorkerOptions {
    pub fn credentials(&self) -> RequestCredentials {
        self.inner.get("credentials").as_::<RequestCredentials>()
    }

    pub fn set_credentials(&mut self, value: &RequestCredentials) {
        self.inner.set("credentials", value);
    }
}
impl WorkerOptions {
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
