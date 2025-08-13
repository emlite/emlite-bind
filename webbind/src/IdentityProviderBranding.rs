use super::*;




/// The IdentityProviderBranding dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdentityProviderBranding {
    inner: Any,
}

impl FromVal for IdentityProviderBranding {
    fn from_val(v: &Any) -> Self {
        IdentityProviderBranding { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for IdentityProviderBranding {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for IdentityProviderBranding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for IdentityProviderBranding {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for IdentityProviderBranding {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<IdentityProviderBranding> for Any {
    fn from(s: IdentityProviderBranding) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&IdentityProviderBranding> for Any {
    fn from(s: &IdentityProviderBranding) -> Any {
        s.inner.clone()
    }
}

impl IdentityProviderBranding {
    /// Getter of the `background_color` attribute.
    pub fn background_color(&self) -> JsString {
        self.inner.get("background_color").as_::<JsString>()
    }

    /// Setter of the `background_color` attribute.
    pub fn set_background_color(&mut self, value: &JsString) {
        self.inner.set("background_color", value);
    }
}
impl IdentityProviderBranding {
    /// Getter of the `color` attribute.
    pub fn color(&self) -> JsString {
        self.inner.get("color").as_::<JsString>()
    }

    /// Setter of the `color` attribute.
    pub fn set_color(&mut self, value: &JsString) {
        self.inner.set("color", value);
    }
}
impl IdentityProviderBranding {
    /// Getter of the `icons` attribute.
    pub fn icons(&self) -> TypedArray<IdentityProviderIcon> {
        self.inner.get("icons").as_::<TypedArray<IdentityProviderIcon>>()
    }

    /// Setter of the `icons` attribute.
    pub fn set_icons(&mut self, value: &TypedArray<IdentityProviderIcon>) {
        self.inner.set("icons", value);
    }
}
impl IdentityProviderBranding {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
