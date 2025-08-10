use super::*;

/// The XRSessionInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSessionInit {
    inner: Any,
}

impl FromVal for XRSessionInit {
    fn from_val(v: &Any) -> Self {
        XRSessionInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRSessionInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRSessionInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRSessionInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRSessionInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRSessionInit> for Any {
    fn from(s: XRSessionInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRSessionInit> for Any {
    fn from(s: &XRSessionInit) -> Any {
        s.inner.clone()
    }
}

impl XRSessionInit {
    /// Getter of the `requiredFeatures` attribute.
    pub fn required_features(&self) -> TypedArray<JsString> {
        self.inner
            .get("requiredFeatures")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `requiredFeatures` attribute.
    pub fn set_required_features(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("requiredFeatures", value);
    }
}
impl XRSessionInit {
    /// Getter of the `optionalFeatures` attribute.
    pub fn optional_features(&self) -> TypedArray<JsString> {
        self.inner
            .get("optionalFeatures")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `optionalFeatures` attribute.
    pub fn set_optional_features(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("optionalFeatures", value);
    }
}
