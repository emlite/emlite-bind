use super::*;




/// The ElementDefinitionOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ElementDefinitionOptions {
    inner: Any,
}

impl FromVal for ElementDefinitionOptions {
    fn from_val(v: &Any) -> Self {
        ElementDefinitionOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ElementDefinitionOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ElementDefinitionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ElementDefinitionOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ElementDefinitionOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ElementDefinitionOptions> for Any {
    fn from(s: ElementDefinitionOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ElementDefinitionOptions> for Any {
    fn from(s: &ElementDefinitionOptions) -> Any {
        s.inner.clone()
    }
}

impl ElementDefinitionOptions {
    /// Getter of the `extends` attribute.
    pub fn extends(&self) -> JsString {
        self.inner.get("extends").as_::<JsString>()
    }

    /// Setter of the `extends` attribute.
    pub fn set_extends(&mut self, value: &JsString) {
        self.inner.set("extends", value);
    }
}
