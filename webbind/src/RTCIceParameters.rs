use super::*;

/// The RTCIceParameters dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceParameters {
    inner: Any,
}

impl FromVal for RTCIceParameters {
    fn from_val(v: &Any) -> Self {
        RTCIceParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIceParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIceParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIceParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIceParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCIceParameters> for Any {
    fn from(s: RTCIceParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIceParameters> for Any {
    fn from(s: &RTCIceParameters) -> Any {
        s.inner.clone()
    }
}

impl RTCIceParameters {
    /// Getter of the `usernameFragment` attribute.
    pub fn username_fragment(&self) -> JsString {
        self.inner.get("usernameFragment").as_::<JsString>()
    }

    /// Setter of the `usernameFragment` attribute.
    pub fn set_username_fragment(&mut self, value: &JsString) {
        self.inner.set("usernameFragment", value);
    }
}
impl RTCIceParameters {
    /// Getter of the `password` attribute.
    pub fn password(&self) -> JsString {
        self.inner.get("password").as_::<JsString>()
    }

    /// Setter of the `password` attribute.
    pub fn set_password(&mut self, value: &JsString) {
        self.inner.set("password", value);
    }
}
