use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InputEventInit {
    inner: Any,
}
impl FromVal for InputEventInit {
    fn from_val(v: &Any) -> Self {
        InputEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for InputEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InputEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for InputEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for InputEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<InputEventInit> for Any {
    fn from(s: InputEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&InputEventInit> for Any {
    fn from(s: &InputEventInit) -> Any {
        s.inner.clone()
    }
}

impl InputEventInit {
    pub fn data(&self) -> JsString {
        self.inner.get("data").as_::<JsString>()
    }

    pub fn set_data(&mut self, value: &JsString) {
        self.inner.set("data", value);
    }
}
impl InputEventInit {
    pub fn is_composing(&self) -> bool {
        self.inner.get("isComposing").as_::<bool>()
    }

    pub fn set_is_composing(&mut self, value: bool) {
        self.inner.set("isComposing", value);
    }
}
impl InputEventInit {
    pub fn input_type(&self) -> JsString {
        self.inner.get("inputType").as_::<JsString>()
    }

    pub fn set_input_type(&mut self, value: &JsString) {
        self.inner.set("inputType", value);
    }
}
