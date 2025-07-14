use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<RTCRtpCapabilities> for emlite::Val {
    fn from(s: RTCRtpCapabilities) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCRtpCapabilities {
    pub fn codecs(&self) -> jsbind::Sequence<RTCRtpCodec> {
        self.inner
            .get("codecs")
            .as_::<jsbind::Sequence<RTCRtpCodec>>()
    }

    pub fn set_codecs(&mut self, value: jsbind::Sequence<RTCRtpCodec>) {
        self.inner.set("codecs", value);
    }
}
impl RTCRtpCapabilities {
    pub fn header_extensions(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("headerExtensions")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_header_extensions(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("headerExtensions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<RTCRtpSendParameters> for emlite::Val {
    fn from(s: RTCRtpSendParameters) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCRtpSendParameters {
    pub fn transaction_id(&self) -> jsbind::DOMString {
        self.inner.get("transactionId").as_::<jsbind::DOMString>()
    }

    pub fn set_transaction_id(&mut self, value: jsbind::DOMString) {
        self.inner.set("transactionId", value);
    }
}
impl RTCRtpSendParameters {
    pub fn encodings(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("encodings")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_encodings(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("encodings", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<RTCSetParameterOptions> for emlite::Val {
    fn from(s: RTCSetParameterOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<RTCRtpSender> for emlite::Val {
    fn from(s: RTCRtpSender) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn get_capabilities(kind: jsbind::DOMString) -> RTCRtpCapabilities {
        emlite::Val::global("rtcrtpsender")
            .call("getCapabilities", &[kind.into()])
            .as_::<RTCRtpCapabilities>()
    }
}
impl RTCRtpSender {
    pub fn set_parameters0(&self, parameters: RTCRtpSendParameters) -> jsbind::Promise {
        self.inner
            .call("setParameters", &[parameters.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn set_parameters1(
        &self,
        parameters: RTCRtpSendParameters,
        set_parameter_options: RTCSetParameterOptions,
    ) -> jsbind::Promise {
        self.inner
            .call(
                "setParameters",
                &[parameters.into(), set_parameter_options.into()],
            )
            .as_::<jsbind::Promise>()
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
    pub fn replace_track(&self, with_track: MediaStreamTrack) -> jsbind::Promise {
        self.inner
            .call("replaceTrack", &[with_track.into()])
            .as_::<jsbind::Promise>()
    }
}
impl RTCRtpSender {
    pub fn set_streams(&self, streams: MediaStream) -> jsbind::Undefined {
        self.inner
            .call("setStreams", &[streams.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl RTCRtpSender {
    pub fn get_stats(&self) -> jsbind::Promise {
        self.inner.call("getStats", &[]).as_::<jsbind::Promise>()
    }
}
impl RTCRtpSender {
    pub fn transform(&self) -> jsbind::Any {
        self.inner.get("transform").as_::<jsbind::Any>()
    }

    pub fn set_transform(&mut self, value: jsbind::Any) {
        self.inner.set("transform", value);
    }
}
impl RTCRtpSender {
    pub fn generate_key_frame0(&self) -> jsbind::Promise {
        self.inner
            .call("generateKeyFrame", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn generate_key_frame1(
        &self,
        rids: jsbind::Sequence<jsbind::DOMString>,
    ) -> jsbind::Promise {
        self.inner
            .call("generateKeyFrame", &[rids.into()])
            .as_::<jsbind::Promise>()
    }
}
impl RTCRtpSender {
    pub fn dtmf(&self) -> RTCDTMFSender {
        self.inner.get("dtmf").as_::<RTCDTMFSender>()
    }
}
