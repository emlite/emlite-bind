use super::*;

/// The ProtectedAudienceUtilities class.
/// [`ProtectedAudienceUtilities`](https://developer.mozilla.org/en-US/docs/Web/API/ProtectedAudienceUtilities)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProtectedAudienceUtilities {
    inner: Any,
}
impl FromVal for ProtectedAudienceUtilities {
    fn from_val(v: &Any) -> Self {
        ProtectedAudienceUtilities {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ProtectedAudienceUtilities {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ProtectedAudienceUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ProtectedAudienceUtilities {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ProtectedAudienceUtilities {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ProtectedAudienceUtilities> for Any {
    fn from(s: ProtectedAudienceUtilities) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ProtectedAudienceUtilities> for Any {
    fn from(s: &ProtectedAudienceUtilities) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ProtectedAudienceUtilities);

impl ProtectedAudienceUtilities {
    /// The encodeUtf8 method.
    /// [`ProtectedAudienceUtilities.encodeUtf8`](https://developer.mozilla.org/en-US/docs/Web/API/ProtectedAudienceUtilities/encodeUtf8)
    pub fn encode_utf8(&self, input: &USVString) -> Uint8Array {
        self.inner
            .call("encodeUtf8", &[input.into()])
            .as_::<Uint8Array>()
    }
}
impl ProtectedAudienceUtilities {
    /// The decodeUtf8 method.
    /// [`ProtectedAudienceUtilities.decodeUtf8`](https://developer.mozilla.org/en-US/docs/Web/API/ProtectedAudienceUtilities/decodeUtf8)
    pub fn decode_utf8(&self, bytes: &Uint8Array) -> USVString {
        self.inner
            .call("decodeUtf8", &[bytes.into()])
            .as_::<USVString>()
    }
}
