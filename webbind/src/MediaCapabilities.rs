use super::*;

/// The MediaCapabilities class.
/// [`MediaCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaCapabilities {
    inner: Any,
}

impl FromVal for MediaCapabilities {
    fn from_val(v: &Any) -> Self {
        MediaCapabilities {
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

impl core::ops::Deref for MediaCapabilities {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaCapabilities {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaCapabilities {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaCapabilities> for Any {
    fn from(s: MediaCapabilities) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaCapabilities> for Any {
    fn from(s: &MediaCapabilities) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaCapabilities);

impl MediaCapabilities {
    /// The decodingInfo method.
    /// [`MediaCapabilities.decodingInfo`](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/decodingInfo)
    pub fn decoding_info(
        &self,
        configuration: &MediaDecodingConfiguration,
    ) -> Promise<MediaCapabilitiesDecodingInfo> {
        self.inner
            .call("decodingInfo", &[configuration.into()])
            .as_::<Promise<MediaCapabilitiesDecodingInfo>>()
    }
}
impl MediaCapabilities {
    /// The encodingInfo method.
    /// [`MediaCapabilities.encodingInfo`](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/encodingInfo)
    pub fn encoding_info(
        &self,
        configuration: &MediaEncodingConfiguration,
    ) -> Promise<MediaCapabilitiesEncodingInfo> {
        self.inner
            .call("encodingInfo", &[configuration.into()])
            .as_::<Promise<MediaCapabilitiesEncodingInfo>>()
    }
}
