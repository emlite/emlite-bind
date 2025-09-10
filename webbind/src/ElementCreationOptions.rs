use super::*;

/// The ElementCreationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ElementCreationOptions {
    inner: Any,
}

impl FromVal for ElementCreationOptions {
    fn from_val(v: &Any) -> Self {
        ElementCreationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ElementCreationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ElementCreationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ElementCreationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ElementCreationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ElementCreationOptions> for Any {
    fn from(s: ElementCreationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ElementCreationOptions> for Any {
    fn from(s: &ElementCreationOptions) -> Any {
        s.inner.clone()
    }
}

impl ElementCreationOptions {
    /// Getter of the `customElementRegistry` attribute.
    pub fn custom_element_registry(&self) -> CustomElementRegistry {
        self.inner
            .get("customElementRegistry")
            .as_::<CustomElementRegistry>()
    }

    /// Setter of the `customElementRegistry` attribute.
    pub fn set_custom_element_registry(&mut self, value: &CustomElementRegistry) {
        self.inner.set("customElementRegistry", value);
    }
}
impl ElementCreationOptions {
    /// Getter of the `is` attribute.
    pub fn is(&self) -> JsString {
        self.inner.get("is").as_::<JsString>()
    }

    /// Setter of the `is` attribute.
    pub fn set_is(&mut self, value: &JsString) {
        self.inner.set("is", value);
    }
}
