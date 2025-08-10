use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StartViewTransitionOptions {
    inner: Any,
}
impl FromVal for StartViewTransitionOptions {
    fn from_val(v: &Any) -> Self {
        StartViewTransitionOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StartViewTransitionOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StartViewTransitionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for StartViewTransitionOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for StartViewTransitionOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<StartViewTransitionOptions> for Any {
    fn from(s: StartViewTransitionOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&StartViewTransitionOptions> for Any {
    fn from(s: &StartViewTransitionOptions) -> Any {
        s.inner.clone()
    }
}

impl StartViewTransitionOptions {
    pub fn update(&self) -> Function {
        self.inner.get("update").as_::<Function>()
    }

    pub fn set_update(&mut self, value: &Function) {
        self.inner.set("update", value);
    }
}
impl StartViewTransitionOptions {
    pub fn types(&self) -> TypedArray<JsString> {
        self.inner.get("types").as_::<TypedArray<JsString>>()
    }

    pub fn set_types(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("types", value);
    }
}
