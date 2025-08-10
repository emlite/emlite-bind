use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaCapabilitiesDecodingInfo {
    inner: Any,
}
impl FromVal for MediaCapabilitiesDecodingInfo {
    fn from_val(v: &Any) -> Self {
        MediaCapabilitiesDecodingInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaCapabilitiesDecodingInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaCapabilitiesDecodingInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaCapabilitiesDecodingInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaCapabilitiesDecodingInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaCapabilitiesDecodingInfo> for Any {
    fn from(s: MediaCapabilitiesDecodingInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaCapabilitiesDecodingInfo> for Any {
    fn from(s: &MediaCapabilitiesDecodingInfo) -> Any {
        s.inner.clone()
    }
}

impl MediaCapabilitiesDecodingInfo {
    pub fn key_system_access(&self) -> MediaKeySystemAccess {
        self.inner
            .get("keySystemAccess")
            .as_::<MediaKeySystemAccess>()
    }

    pub fn set_key_system_access(&mut self, value: &MediaKeySystemAccess) {
        self.inner.set("keySystemAccess", value);
    }
}
impl MediaCapabilitiesDecodingInfo {
    pub fn configuration(&self) -> MediaDecodingConfiguration {
        self.inner
            .get("configuration")
            .as_::<MediaDecodingConfiguration>()
    }

    pub fn set_configuration(&mut self, value: &MediaDecodingConfiguration) {
        self.inner.set("configuration", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaDecodingConfiguration {
    inner: Any,
}
impl FromVal for MediaDecodingConfiguration {
    fn from_val(v: &Any) -> Self {
        MediaDecodingConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaDecodingConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaDecodingConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaDecodingConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaDecodingConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaDecodingConfiguration> for Any {
    fn from(s: MediaDecodingConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaDecodingConfiguration> for Any {
    fn from(s: &MediaDecodingConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MediaDecodingConfiguration {
    pub fn type_(&self) -> MediaDecodingType {
        self.inner.get("type").as_::<MediaDecodingType>()
    }

    pub fn set_type_(&mut self, value: &MediaDecodingType) {
        self.inner.set("type", value);
    }
}
impl MediaDecodingConfiguration {
    pub fn key_system_configuration(&self) -> MediaCapabilitiesKeySystemConfiguration {
        self.inner
            .get("keySystemConfiguration")
            .as_::<MediaCapabilitiesKeySystemConfiguration>()
    }

    pub fn set_key_system_configuration(
        &mut self,
        value: &MediaCapabilitiesKeySystemConfiguration,
    ) {
        self.inner.set("keySystemConfiguration", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaCapabilitiesEncodingInfo {
    inner: Any,
}
impl FromVal for MediaCapabilitiesEncodingInfo {
    fn from_val(v: &Any) -> Self {
        MediaCapabilitiesEncodingInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaCapabilitiesEncodingInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaCapabilitiesEncodingInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaCapabilitiesEncodingInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaCapabilitiesEncodingInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaCapabilitiesEncodingInfo> for Any {
    fn from(s: MediaCapabilitiesEncodingInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaCapabilitiesEncodingInfo> for Any {
    fn from(s: &MediaCapabilitiesEncodingInfo) -> Any {
        s.inner.clone()
    }
}

impl MediaCapabilitiesEncodingInfo {
    pub fn configuration(&self) -> MediaEncodingConfiguration {
        self.inner
            .get("configuration")
            .as_::<MediaEncodingConfiguration>()
    }

    pub fn set_configuration(&mut self, value: &MediaEncodingConfiguration) {
        self.inner.set("configuration", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaEncodingConfiguration {
    inner: Any,
}
impl FromVal for MediaEncodingConfiguration {
    fn from_val(v: &Any) -> Self {
        MediaEncodingConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaEncodingConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaEncodingConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaEncodingConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaEncodingConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaEncodingConfiguration> for Any {
    fn from(s: MediaEncodingConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaEncodingConfiguration> for Any {
    fn from(s: &MediaEncodingConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MediaEncodingConfiguration {
    pub fn type_(&self) -> MediaEncodingType {
        self.inner.get("type").as_::<MediaEncodingType>()
    }

    pub fn set_type_(&mut self, value: &MediaEncodingType) {
        self.inner.set("type", value);
    }
}
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
