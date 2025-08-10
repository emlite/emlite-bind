use super::*;

/// The FunctionParameter dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FunctionParameter {
    inner: Any,
}

impl FromVal for FunctionParameter {
    fn from_val(v: &Any) -> Self {
        FunctionParameter { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FunctionParameter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FunctionParameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FunctionParameter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FunctionParameter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FunctionParameter> for Any {
    fn from(s: FunctionParameter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FunctionParameter> for Any {
    fn from(s: &FunctionParameter) -> Any {
        s.inner.clone()
    }
}

impl FunctionParameter {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl FunctionParameter {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl FunctionParameter {
    /// Getter of the `defaultValue` attribute.
    pub fn default_value(&self) -> JsString {
        self.inner.get("defaultValue").as_::<JsString>()
    }

    /// Setter of the `defaultValue` attribute.
    pub fn set_default_value(&mut self, value: &JsString) {
        self.inner.set("defaultValue", value);
    }
}
