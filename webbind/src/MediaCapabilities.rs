use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaCapabilitiesDecodingInfo {
    inner: emlite::Val,
}
impl FromVal for MediaCapabilitiesDecodingInfo {
    fn from_val(v: &emlite::Val) -> Self {
        MediaCapabilitiesDecodingInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaCapabilitiesDecodingInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaCapabilitiesDecodingInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaCapabilitiesDecodingInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaCapabilitiesDecodingInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaCapabilitiesDecodingInfo> for emlite::Val {
    fn from(s: MediaCapabilitiesDecodingInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaCapabilitiesDecodingInfo {
    pub fn key_system_access(&self) -> MediaKeySystemAccess {
        self.inner
            .get("keySystemAccess")
            .as_::<MediaKeySystemAccess>()
    }

    pub fn set_key_system_access(&mut self, value: MediaKeySystemAccess) {
        self.inner.set("keySystemAccess", value);
    }
}
impl MediaCapabilitiesDecodingInfo {
    pub fn configuration(&self) -> MediaDecodingConfiguration {
        self.inner
            .get("configuration")
            .as_::<MediaDecodingConfiguration>()
    }

    pub fn set_configuration(&mut self, value: MediaDecodingConfiguration) {
        self.inner.set("configuration", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaDecodingConfiguration {
    inner: emlite::Val,
}
impl FromVal for MediaDecodingConfiguration {
    fn from_val(v: &emlite::Val) -> Self {
        MediaDecodingConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaDecodingConfiguration {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaDecodingConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaDecodingConfiguration {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaDecodingConfiguration {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaDecodingConfiguration> for emlite::Val {
    fn from(s: MediaDecodingConfiguration) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaDecodingConfiguration {
    pub fn type_(&self) -> MediaDecodingType {
        self.inner.get("type").as_::<MediaDecodingType>()
    }

    pub fn set_type_(&mut self, value: MediaDecodingType) {
        self.inner.set("type", value);
    }
}
impl MediaDecodingConfiguration {
    pub fn key_system_configuration(&self) -> jsbind::Any {
        self.inner
            .get("keySystemConfiguration")
            .as_::<jsbind::Any>()
    }

    pub fn set_key_system_configuration(&mut self, value: jsbind::Any) {
        self.inner.set("keySystemConfiguration", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaCapabilitiesEncodingInfo {
    inner: emlite::Val,
}
impl FromVal for MediaCapabilitiesEncodingInfo {
    fn from_val(v: &emlite::Val) -> Self {
        MediaCapabilitiesEncodingInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaCapabilitiesEncodingInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaCapabilitiesEncodingInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaCapabilitiesEncodingInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaCapabilitiesEncodingInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaCapabilitiesEncodingInfo> for emlite::Val {
    fn from(s: MediaCapabilitiesEncodingInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaCapabilitiesEncodingInfo {
    pub fn configuration(&self) -> MediaEncodingConfiguration {
        self.inner
            .get("configuration")
            .as_::<MediaEncodingConfiguration>()
    }

    pub fn set_configuration(&mut self, value: MediaEncodingConfiguration) {
        self.inner.set("configuration", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaEncodingConfiguration {
    inner: emlite::Val,
}
impl FromVal for MediaEncodingConfiguration {
    fn from_val(v: &emlite::Val) -> Self {
        MediaEncodingConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaEncodingConfiguration {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaEncodingConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaEncodingConfiguration {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaEncodingConfiguration {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaEncodingConfiguration> for emlite::Val {
    fn from(s: MediaEncodingConfiguration) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaEncodingConfiguration {
    pub fn type_(&self) -> MediaEncodingType {
        self.inner.get("type").as_::<MediaEncodingType>()
    }

    pub fn set_type_(&mut self, value: MediaEncodingType) {
        self.inner.set("type", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaCapabilities {
    inner: emlite::Val,
}
impl FromVal for MediaCapabilities {
    fn from_val(v: &emlite::Val) -> Self {
        MediaCapabilities {
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
impl core::ops::Deref for MediaCapabilities {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaCapabilities {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaCapabilities {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaCapabilities> for emlite::Val {
    fn from(s: MediaCapabilities) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MediaCapabilities);

impl MediaCapabilities {
    pub fn decoding_info(&self, configuration: MediaDecodingConfiguration) -> jsbind::Promise {
        self.inner
            .call("decodingInfo", &[configuration.into()])
            .as_::<jsbind::Promise>()
    }
}
impl MediaCapabilities {
    pub fn encoding_info(&self, configuration: MediaEncodingConfiguration) -> jsbind::Promise {
        self.inner
            .call("encodingInfo", &[configuration.into()])
            .as_::<jsbind::Promise>()
    }
}
