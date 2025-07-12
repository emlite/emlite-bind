use super::*;

#[derive(Clone, Debug)]
pub struct ProtectedAudienceUtilities {
    inner: emlite::Val,
}
impl FromVal for ProtectedAudienceUtilities {
    fn from_val(v: &emlite::Val) -> Self {
        ProtectedAudienceUtilities {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ProtectedAudienceUtilities {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ProtectedAudienceUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ProtectedAudienceUtilities> for emlite::Val {
    fn from(s: ProtectedAudienceUtilities) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ProtectedAudienceUtilities {
    pub fn encode_utf8(&self, input: jsbind::USVString) -> jsbind::Uint8Array {
        self.inner
            .call("encodeUtf8", &[input.into()])
            .as_::<jsbind::Uint8Array>()
    }
}
impl ProtectedAudienceUtilities {
    pub fn decode_utf8(&self, bytes: jsbind::Uint8Array) -> jsbind::USVString {
        self.inner
            .call("decodeUtf8", &[bytes.into()])
            .as_::<jsbind::USVString>()
    }
}
