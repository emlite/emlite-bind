use super::*;




/// The PropertyDefinition dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PropertyDefinition {
    inner: Any,
}

impl FromVal for PropertyDefinition {
    fn from_val(v: &Any) -> Self {
        PropertyDefinition { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PropertyDefinition {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PropertyDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PropertyDefinition {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PropertyDefinition {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PropertyDefinition> for Any {
    fn from(s: PropertyDefinition) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PropertyDefinition> for Any {
    fn from(s: &PropertyDefinition) -> Any {
        s.inner.clone()
    }
}

impl PropertyDefinition {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl PropertyDefinition {
    /// Getter of the `syntax` attribute.
    pub fn syntax(&self) -> JsString {
        self.inner.get("syntax").as_::<JsString>()
    }

    /// Setter of the `syntax` attribute.
    pub fn set_syntax(&mut self, value: &JsString) {
        self.inner.set("syntax", value);
    }
}
impl PropertyDefinition {
    /// Getter of the `inherits` attribute.
    pub fn inherits(&self) -> bool {
        self.inner.get("inherits").as_::<bool>()
    }

    /// Setter of the `inherits` attribute.
    pub fn set_inherits(&mut self, value: bool) {
        self.inner.set("inherits", value);
    }
}
impl PropertyDefinition {
    /// Getter of the `initialValue` attribute.
    pub fn initial_value(&self) -> JsString {
        self.inner.get("initialValue").as_::<JsString>()
    }

    /// Setter of the `initialValue` attribute.
    pub fn set_initial_value(&mut self, value: &JsString) {
        self.inner.set("initialValue", value);
    }
}
