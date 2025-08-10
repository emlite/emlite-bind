use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRPermissionDescriptor {
    inner: Any,
}
impl FromVal for XRPermissionDescriptor {
    fn from_val(v: &Any) -> Self {
        XRPermissionDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRPermissionDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRPermissionDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRPermissionDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRPermissionDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRPermissionDescriptor> for Any {
    fn from(s: XRPermissionDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRPermissionDescriptor> for Any {
    fn from(s: &XRPermissionDescriptor) -> Any {
        s.inner.clone()
    }
}

impl XRPermissionDescriptor {
    pub fn mode(&self) -> XRSessionMode {
        self.inner.get("mode").as_::<XRSessionMode>()
    }

    pub fn set_mode(&mut self, value: &XRSessionMode) {
        self.inner.set("mode", value);
    }
}
impl XRPermissionDescriptor {
    pub fn required_features(&self) -> TypedArray<JsString> {
        self.inner
            .get("requiredFeatures")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_required_features(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("requiredFeatures", value);
    }
}
impl XRPermissionDescriptor {
    pub fn optional_features(&self) -> TypedArray<JsString> {
        self.inner
            .get("optionalFeatures")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_optional_features(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("optionalFeatures", value);
    }
}
