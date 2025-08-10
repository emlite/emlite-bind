use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TransitionEventInit {
    inner: Any,
}
impl FromVal for TransitionEventInit {
    fn from_val(v: &Any) -> Self {
        TransitionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TransitionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TransitionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TransitionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TransitionEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TransitionEventInit> for Any {
    fn from(s: TransitionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TransitionEventInit> for Any {
    fn from(s: &TransitionEventInit) -> Any {
        s.inner.clone()
    }
}

impl TransitionEventInit {
    pub fn property_name(&self) -> JsString {
        self.inner.get("propertyName").as_::<JsString>()
    }

    pub fn set_property_name(&mut self, value: &JsString) {
        self.inner.set("propertyName", value);
    }
}
impl TransitionEventInit {
    pub fn elapsed_time(&self) -> f64 {
        self.inner.get("elapsedTime").as_::<f64>()
    }

    pub fn set_elapsed_time(&mut self, value: f64) {
        self.inner.set("elapsedTime", value);
    }
}
impl TransitionEventInit {
    pub fn pseudo_element(&self) -> JsString {
        self.inner.get("pseudoElement").as_::<JsString>()
    }

    pub fn set_pseudo_element(&mut self, value: &JsString) {
        self.inner.set("pseudoElement", value);
    }
}
