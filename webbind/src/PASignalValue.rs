use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PASignalValue {
    inner: Any,
}
impl FromVal for PASignalValue {
    fn from_val(v: &Any) -> Self {
        PASignalValue { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PASignalValue {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PASignalValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PASignalValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PASignalValue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PASignalValue> for Any {
    fn from(s: PASignalValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PASignalValue> for Any {
    fn from(s: &PASignalValue) -> Any {
        s.inner.clone()
    }
}

impl PASignalValue {
    pub fn base_value(&self) -> JsString {
        self.inner.get("baseValue").as_::<JsString>()
    }

    pub fn set_base_value(&mut self, value: &JsString) {
        self.inner.set("baseValue", value);
    }
}
impl PASignalValue {
    pub fn scale(&self) -> f64 {
        self.inner.get("scale").as_::<f64>()
    }

    pub fn set_scale(&mut self, value: f64) {
        self.inner.set("scale", value);
    }
}
impl PASignalValue {
    pub fn offset(&self) -> Any {
        self.inner.get("offset").as_::<Any>()
    }

    pub fn set_offset(&mut self, value: &Any) {
        self.inner.set("offset", value);
    }
}
