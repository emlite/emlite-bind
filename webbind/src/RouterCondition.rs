use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RouterCondition {
    inner: Any,
}
impl FromVal for RouterCondition {
    fn from_val(v: &Any) -> Self {
        RouterCondition { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RouterCondition {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RouterCondition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RouterCondition {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RouterCondition {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RouterCondition> for Any {
    fn from(s: RouterCondition) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RouterCondition> for Any {
    fn from(s: &RouterCondition) -> Any {
        s.inner.clone()
    }
}

impl RouterCondition {
    pub fn url_pattern(&self) -> Any {
        self.inner.get("urlPattern").as_::<Any>()
    }

    pub fn set_url_pattern(&mut self, value: &Any) {
        self.inner.set("urlPattern", value);
    }
}
impl RouterCondition {
    pub fn request_method(&self) -> JsString {
        self.inner.get("requestMethod").as_::<JsString>()
    }

    pub fn set_request_method(&mut self, value: &JsString) {
        self.inner.set("requestMethod", value);
    }
}
impl RouterCondition {
    pub fn request_mode(&self) -> RequestMode {
        self.inner.get("requestMode").as_::<RequestMode>()
    }

    pub fn set_request_mode(&mut self, value: &RequestMode) {
        self.inner.set("requestMode", value);
    }
}
impl RouterCondition {
    pub fn request_destination(&self) -> RequestDestination {
        self.inner
            .get("requestDestination")
            .as_::<RequestDestination>()
    }

    pub fn set_request_destination(&mut self, value: &RequestDestination) {
        self.inner.set("requestDestination", value);
    }
}
impl RouterCondition {
    pub fn running_status(&self) -> RunningStatus {
        self.inner.get("runningStatus").as_::<RunningStatus>()
    }

    pub fn set_running_status(&mut self, value: &RunningStatus) {
        self.inner.set("runningStatus", value);
    }
}
impl RouterCondition {
    pub fn or(&self) -> TypedArray<RouterCondition> {
        self.inner.get("or").as_::<TypedArray<RouterCondition>>()
    }

    pub fn set_or(&mut self, value: &TypedArray<RouterCondition>) {
        self.inner.set("or", value);
    }
}
impl RouterCondition {
    pub fn not(&self) -> RouterCondition {
        self.inner.get("not").as_::<RouterCondition>()
    }

    pub fn set_not(&mut self, value: &RouterCondition) {
        self.inner.set("not", value);
    }
}
