use super::*;

/// The KeySystemTrackConfiguration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct KeySystemTrackConfiguration {
    inner: Any,
}

impl FromVal for KeySystemTrackConfiguration {
    fn from_val(v: &Any) -> Self {
        KeySystemTrackConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for KeySystemTrackConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for KeySystemTrackConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for KeySystemTrackConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for KeySystemTrackConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<KeySystemTrackConfiguration> for Any {
    fn from(s: KeySystemTrackConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&KeySystemTrackConfiguration> for Any {
    fn from(s: &KeySystemTrackConfiguration) -> Any {
        s.inner.clone()
    }
}

impl KeySystemTrackConfiguration {
    /// Getter of the `robustness` attribute.
    pub fn robustness(&self) -> JsString {
        self.inner.get("robustness").as_::<JsString>()
    }

    /// Setter of the `robustness` attribute.
    pub fn set_robustness(&mut self, value: &JsString) {
        self.inner.set("robustness", value);
    }
}
impl KeySystemTrackConfiguration {
    /// Getter of the `encryptionScheme` attribute.
    pub fn encryption_scheme(&self) -> JsString {
        self.inner.get("encryptionScheme").as_::<JsString>()
    }

    /// Setter of the `encryptionScheme` attribute.
    pub fn set_encryption_scheme(&mut self, value: &JsString) {
        self.inner.set("encryptionScheme", value);
    }
}
