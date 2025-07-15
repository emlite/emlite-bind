use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpCapabilities {
    inner: emlite::Val,
}
impl FromVal for RTCRtpCapabilities {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpCapabilities { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpCapabilities {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCRtpCapabilities {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCRtpCapabilities {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCRtpCapabilities> for emlite::Val {
    fn from(s: RTCRtpCapabilities) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCRtpCapabilities> for emlite::Val {
    fn from(s: &RTCRtpCapabilities) -> emlite::Val {
        s.inner.clone()
    }
}

impl RTCRtpCapabilities {
    pub fn codecs(&self) -> Sequence<RTCRtpCodec> {
        self.inner.get("codecs").as_::<Sequence<RTCRtpCodec>>()
    }

    pub fn set_codecs(&mut self, value: Sequence<RTCRtpCodec>) {
        self.inner.set("codecs", value);
    }
}
impl RTCRtpCapabilities {
    pub fn header_extensions(&self) -> Sequence<Any> {
        self.inner.get("headerExtensions").as_::<Sequence<Any>>()
    }

    pub fn set_header_extensions(&mut self, value: Sequence<Any>) {
        self.inner.set("headerExtensions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpSendParameters {
    inner: emlite::Val,
}
impl FromVal for RTCRtpSendParameters {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpSendParameters { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpSendParameters {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpSendParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCRtpSendParameters {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCRtpSendParameters {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCRtpSendParameters> for emlite::Val {
    fn from(s: RTCRtpSendParameters) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCRtpSendParameters> for emlite::Val {
    fn from(s: &RTCRtpSendParameters) -> emlite::Val {
        s.inner.clone()
    }
}

impl RTCRtpSendParameters {
    pub fn transaction_id(&self) -> DOMString {
        self.inner.get("transactionId").as_::<DOMString>()
    }

    pub fn set_transaction_id(&mut self, value: DOMString) {
        self.inner.set("transactionId", value);
    }
}
impl RTCRtpSendParameters {
    pub fn encodings(&self) -> Sequence<Any> {
        self.inner.get("encodings").as_::<Sequence<Any>>()
    }

    pub fn set_encodings(&mut self, value: Sequence<Any>) {
        self.inner.set("encodings", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCSetParameterOptions {
    inner: emlite::Val,
}
impl FromVal for RTCSetParameterOptions {
    fn from_val(v: &emlite::Val) -> Self {
        RTCSetParameterOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCSetParameterOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCSetParameterOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCSetParameterOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCSetParameterOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCSetParameterOptions> for emlite::Val {
    fn from(s: RTCSetParameterOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCSetParameterOptions> for emlite::Val {
    fn from(s: &RTCSetParameterOptions) -> emlite::Val {
        s.inner.clone()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpSender {
    inner: emlite::Val,
}
impl FromVal for RTCRtpSender {
    fn from_val(v: &emlite::Val) -> Self {
        RTCRtpSender {
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
impl core::ops::Deref for RTCRtpSender {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCRtpSender {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCRtpSender {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCRtpSender> for emlite::Val {
    fn from(s: RTCRtpSender) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCRtpSender> for emlite::Val {
    fn from(s: &RTCRtpSender) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCRtpSender);

impl RTCRtpSender {
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}
impl RTCRtpSender {
    pub fn transport(&self) -> RTCDtlsTransport {
        self.inner.get("transport").as_::<RTCDtlsTransport>()
    }
}
impl RTCRtpSender {
    pub fn get_capabilities(kind: DOMString) -> RTCRtpCapabilities {
        emlite::Val::global("RTCRtpSender")
            .call("getCapabilities", &[kind.into()])
            .as_::<RTCRtpCapabilities>()
    }
}
impl RTCRtpSender {
    pub fn set_parameters0(&self, parameters: RTCRtpSendParameters) -> Promise {
        self.inner
            .call("setParameters", &[parameters.into()])
            .as_::<Promise>()
    }

    pub fn set_parameters1(
        &self,
        parameters: RTCRtpSendParameters,
        set_parameter_options: RTCSetParameterOptions,
    ) -> Promise {
        self.inner
            .call(
                "setParameters",
                &[parameters.into(), set_parameter_options.into()],
            )
            .as_::<Promise>()
    }
}
impl RTCRtpSender {
    pub fn get_parameters(&self) -> RTCRtpSendParameters {
        self.inner
            .call("getParameters", &[])
            .as_::<RTCRtpSendParameters>()
    }
}
impl RTCRtpSender {
    pub fn replace_track(&self, with_track: MediaStreamTrack) -> Promise {
        self.inner
            .call("replaceTrack", &[with_track.into()])
            .as_::<Promise>()
    }
}
impl RTCRtpSender {
    pub fn set_streams(&self, streams: MediaStream) -> Undefined {
        self.inner
            .call("setStreams", &[streams.into()])
            .as_::<Undefined>()
    }
}
impl RTCRtpSender {
    pub fn get_stats(&self) -> Promise {
        self.inner.call("getStats", &[]).as_::<Promise>()
    }
}
impl RTCRtpSender {
    pub fn transform(&self) -> Any {
        self.inner.get("transform").as_::<Any>()
    }

    pub fn set_transform(&mut self, value: Any) {
        self.inner.set("transform", value);
    }
}
impl RTCRtpSender {
    pub fn generate_key_frame0(&self) -> Promise {
        self.inner.call("generateKeyFrame", &[]).as_::<Promise>()
    }

    pub fn generate_key_frame1(&self, rids: Sequence<DOMString>) -> Promise {
        self.inner
            .call("generateKeyFrame", &[rids.into()])
            .as_::<Promise>()
    }
}
impl RTCRtpSender {
    pub fn dtmf(&self) -> RTCDTMFSender {
        self.inner.get("dtmf").as_::<RTCDTMFSender>()
    }
}
