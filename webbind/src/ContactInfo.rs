use super::*;

/// The ContactInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ContactInfo {
    inner: Any,
}

impl FromVal for ContactInfo {
    fn from_val(v: &Any) -> Self {
        ContactInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ContactInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ContactInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ContactInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ContactInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ContactInfo> for Any {
    fn from(s: ContactInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ContactInfo> for Any {
    fn from(s: &ContactInfo) -> Any {
        s.inner.clone()
    }
}

impl ContactInfo {
    /// Getter of the `address` attribute.
    pub fn address(&self) -> TypedArray<ContactAddress> {
        self.inner
            .get("address")
            .as_::<TypedArray<ContactAddress>>()
    }

    /// Setter of the `address` attribute.
    pub fn set_address(&mut self, value: &TypedArray<ContactAddress>) {
        self.inner.set("address", value);
    }
}
impl ContactInfo {
    /// Getter of the `email` attribute.
    pub fn email(&self) -> TypedArray<JsString> {
        self.inner.get("email").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `email` attribute.
    pub fn set_email(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("email", value);
    }
}
impl ContactInfo {
    /// Getter of the `icon` attribute.
    pub fn icon(&self) -> TypedArray<Blob> {
        self.inner.get("icon").as_::<TypedArray<Blob>>()
    }

    /// Setter of the `icon` attribute.
    pub fn set_icon(&mut self, value: &TypedArray<Blob>) {
        self.inner.set("icon", value);
    }
}
impl ContactInfo {
    /// Getter of the `name` attribute.
    pub fn name(&self) -> TypedArray<JsString> {
        self.inner.get("name").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `name` attribute.
    pub fn set_name(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("name", value);
    }
}
impl ContactInfo {
    /// Getter of the `tel` attribute.
    pub fn tel(&self) -> TypedArray<JsString> {
        self.inner.get("tel").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `tel` attribute.
    pub fn set_tel(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("tel", value);
    }
}
