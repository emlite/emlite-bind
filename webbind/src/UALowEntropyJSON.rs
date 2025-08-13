use super::*;




/// The UALowEntropyJSON dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UALowEntropyJSON {
    inner: Any,
}

impl FromVal for UALowEntropyJSON {
    fn from_val(v: &Any) -> Self {
        UALowEntropyJSON { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for UALowEntropyJSON {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for UALowEntropyJSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for UALowEntropyJSON {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for UALowEntropyJSON {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<UALowEntropyJSON> for Any {
    fn from(s: UALowEntropyJSON) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&UALowEntropyJSON> for Any {
    fn from(s: &UALowEntropyJSON) -> Any {
        s.inner.clone()
    }
}

impl UALowEntropyJSON {
    /// Getter of the `brands` attribute.
    pub fn brands(&self) -> TypedArray<NavigatorUABrandVersion> {
        self.inner.get("brands").as_::<TypedArray<NavigatorUABrandVersion>>()
    }

    /// Setter of the `brands` attribute.
    pub fn set_brands(&mut self, value: &TypedArray<NavigatorUABrandVersion>) {
        self.inner.set("brands", value);
    }
}
impl UALowEntropyJSON {
    /// Getter of the `mobile` attribute.
    pub fn mobile(&self) -> bool {
        self.inner.get("mobile").as_::<bool>()
    }

    /// Setter of the `mobile` attribute.
    pub fn set_mobile(&mut self, value: bool) {
        self.inner.set("mobile", value);
    }
}
impl UALowEntropyJSON {
    /// Getter of the `platform` attribute.
    pub fn platform(&self) -> JsString {
        self.inner.get("platform").as_::<JsString>()
    }

    /// Setter of the `platform` attribute.
    pub fn set_platform(&mut self, value: &JsString) {
        self.inner.set("platform", value);
    }
}
