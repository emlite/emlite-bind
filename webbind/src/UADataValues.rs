use super::*;




/// The UADataValues dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UADataValues {
    inner: Any,
}

impl FromVal for UADataValues {
    fn from_val(v: &Any) -> Self {
        UADataValues { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for UADataValues {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for UADataValues {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for UADataValues {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for UADataValues {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<UADataValues> for Any {
    fn from(s: UADataValues) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&UADataValues> for Any {
    fn from(s: &UADataValues) -> Any {
        s.inner.clone()
    }
}

impl UADataValues {
    /// Getter of the `architecture` attribute.
    pub fn architecture(&self) -> JsString {
        self.inner.get("architecture").as_::<JsString>()
    }

    /// Setter of the `architecture` attribute.
    pub fn set_architecture(&mut self, value: &JsString) {
        self.inner.set("architecture", value);
    }
}
impl UADataValues {
    /// Getter of the `bitness` attribute.
    pub fn bitness(&self) -> JsString {
        self.inner.get("bitness").as_::<JsString>()
    }

    /// Setter of the `bitness` attribute.
    pub fn set_bitness(&mut self, value: &JsString) {
        self.inner.set("bitness", value);
    }
}
impl UADataValues {
    /// Getter of the `brands` attribute.
    pub fn brands(&self) -> TypedArray<NavigatorUABrandVersion> {
        self.inner.get("brands").as_::<TypedArray<NavigatorUABrandVersion>>()
    }

    /// Setter of the `brands` attribute.
    pub fn set_brands(&mut self, value: &TypedArray<NavigatorUABrandVersion>) {
        self.inner.set("brands", value);
    }
}
impl UADataValues {
    /// Getter of the `formFactors` attribute.
    pub fn form_factors(&self) -> TypedArray<JsString> {
        self.inner.get("formFactors").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `formFactors` attribute.
    pub fn set_form_factors(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("formFactors", value);
    }
}
impl UADataValues {
    /// Getter of the `fullVersionList` attribute.
    pub fn full_version_list(&self) -> TypedArray<NavigatorUABrandVersion> {
        self.inner.get("fullVersionList").as_::<TypedArray<NavigatorUABrandVersion>>()
    }

    /// Setter of the `fullVersionList` attribute.
    pub fn set_full_version_list(&mut self, value: &TypedArray<NavigatorUABrandVersion>) {
        self.inner.set("fullVersionList", value);
    }
}
impl UADataValues {
    /// Getter of the `model` attribute.
    pub fn model(&self) -> JsString {
        self.inner.get("model").as_::<JsString>()
    }

    /// Setter of the `model` attribute.
    pub fn set_model(&mut self, value: &JsString) {
        self.inner.set("model", value);
    }
}
impl UADataValues {
    /// Getter of the `mobile` attribute.
    pub fn mobile(&self) -> bool {
        self.inner.get("mobile").as_::<bool>()
    }

    /// Setter of the `mobile` attribute.
    pub fn set_mobile(&mut self, value: bool) {
        self.inner.set("mobile", value);
    }
}
impl UADataValues {
    /// Getter of the `platform` attribute.
    pub fn platform(&self) -> JsString {
        self.inner.get("platform").as_::<JsString>()
    }

    /// Setter of the `platform` attribute.
    pub fn set_platform(&mut self, value: &JsString) {
        self.inner.set("platform", value);
    }
}
impl UADataValues {
    /// Getter of the `platformVersion` attribute.
    pub fn platform_version(&self) -> JsString {
        self.inner.get("platformVersion").as_::<JsString>()
    }

    /// Setter of the `platformVersion` attribute.
    pub fn set_platform_version(&mut self, value: &JsString) {
        self.inner.set("platformVersion", value);
    }
}
impl UADataValues {
    /// Getter of the `uaFullVersion` attribute.
    pub fn ua_full_version(&self) -> JsString {
        self.inner.get("uaFullVersion").as_::<JsString>()
    }

    /// Setter of the `uaFullVersion` attribute.
    pub fn set_ua_full_version(&mut self, value: &JsString) {
        self.inner.set("uaFullVersion", value);
    }
}
impl UADataValues {
    /// Getter of the `wow64` attribute.
    pub fn wow64(&self) -> bool {
        self.inner.get("wow64").as_::<bool>()
    }

    /// Setter of the `wow64` attribute.
    pub fn set_wow64(&mut self, value: bool) {
        self.inner.set("wow64", value);
    }
}
