use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpCapabilities {
    inner: Any,
}
impl FromVal for RTCRtpCapabilities {
    fn from_val(v: &Any) -> Self {
        RTCRtpCapabilities { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpCapabilities {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtpCapabilities {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtpCapabilities {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtpCapabilities> for Any {
    fn from(s: RTCRtpCapabilities) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtpCapabilities> for Any {
    fn from(s: &RTCRtpCapabilities) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpCapabilities {
    pub fn codecs(&self) -> TypedArray<RTCRtpCodec> {
        self.inner.get("codecs").as_::<TypedArray<RTCRtpCodec>>()
    }

    pub fn set_codecs(&mut self, value: &TypedArray<RTCRtpCodec>) {
        self.inner.set("codecs", value);
    }
}
impl RTCRtpCapabilities {
    pub fn header_extensions(&self) -> TypedArray<Any> {
        self.inner.get("headerExtensions").as_::<TypedArray<Any>>()
    }

    pub fn set_header_extensions(&mut self, value: &TypedArray<Any>) {
        self.inner.set("headerExtensions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpSendParameters {
    inner: Any,
}
impl FromVal for RTCRtpSendParameters {
    fn from_val(v: &Any) -> Self {
        RTCRtpSendParameters { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCRtpSendParameters {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpSendParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtpSendParameters {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtpSendParameters {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtpSendParameters> for Any {
    fn from(s: RTCRtpSendParameters) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtpSendParameters> for Any {
    fn from(s: &RTCRtpSendParameters) -> Any {
        s.inner.clone()
    }
}

impl RTCRtpSendParameters {
    pub fn transaction_id(&self) -> JsString {
        self.inner.get("transactionId").as_::<JsString>()
    }

    pub fn set_transaction_id(&mut self, value: &JsString) {
        self.inner.set("transactionId", value);
    }
}
impl RTCRtpSendParameters {
    pub fn encodings(&self) -> TypedArray<Any> {
        self.inner.get("encodings").as_::<TypedArray<Any>>()
    }

    pub fn set_encodings(&mut self, value: &TypedArray<Any>) {
        self.inner.set("encodings", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCSetParameterOptions {
    inner: Any,
}
impl FromVal for RTCSetParameterOptions {
    fn from_val(v: &Any) -> Self {
        RTCSetParameterOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCSetParameterOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCSetParameterOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCSetParameterOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCSetParameterOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCSetParameterOptions> for Any {
    fn from(s: RTCSetParameterOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCSetParameterOptions> for Any {
    fn from(s: &RTCSetParameterOptions) -> Any {
        s.inner.clone()
    }
}

/// The RTCRtpSender class.
/// [`RTCRtpSender`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCRtpSender {
    inner: Any,
}
impl FromVal for RTCRtpSender {
    fn from_val(v: &Any) -> Self {
        RTCRtpSender {
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
impl core::ops::Deref for RTCRtpSender {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCRtpSender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCRtpSender {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCRtpSender {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCRtpSender> for Any {
    fn from(s: RTCRtpSender) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCRtpSender> for Any {
    fn from(s: &RTCRtpSender) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCRtpSender);

impl RTCRtpSender {
    /// Getter of the `track` attribute.
    /// [`RTCRtpSender.track`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/track)
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }
}
impl RTCRtpSender {
    /// Getter of the `transport` attribute.
    /// [`RTCRtpSender.transport`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transport)
    pub fn transport(&self) -> RTCDtlsTransport {
        self.inner.get("transport").as_::<RTCDtlsTransport>()
    }
}
impl RTCRtpSender {
    /// The getCapabilities method.
    /// [`RTCRtpSender.getCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getCapabilities)
    pub fn get_capabilities(kind: &JsString) -> RTCRtpCapabilities {
        Any::global("RTCRtpSender")
            .call("getCapabilities", &[kind.into()])
            .as_::<RTCRtpCapabilities>()
    }
}
impl RTCRtpSender {
    /// The setParameters method.
    /// [`RTCRtpSender.setParameters`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)
    pub fn set_parameters0(&self, parameters: &RTCRtpSendParameters) -> Promise<Undefined> {
        self.inner
            .call("setParameters", &[parameters.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The setParameters method.
    /// [`RTCRtpSender.setParameters`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)
    pub fn set_parameters1(
        &self,
        parameters: &RTCRtpSendParameters,
        set_parameter_options: &RTCSetParameterOptions,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "setParameters",
                &[parameters.into(), set_parameter_options.into()],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl RTCRtpSender {
    /// The getParameters method.
    /// [`RTCRtpSender.getParameters`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getParameters)
    pub fn get_parameters(&self) -> RTCRtpSendParameters {
        self.inner
            .call("getParameters", &[])
            .as_::<RTCRtpSendParameters>()
    }
}
impl RTCRtpSender {
    /// The replaceTrack method.
    /// [`RTCRtpSender.replaceTrack`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/replaceTrack)
    pub fn replace_track(&self, with_track: &MediaStreamTrack) -> Promise<Undefined> {
        self.inner
            .call("replaceTrack", &[with_track.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl RTCRtpSender {
    /// The setStreams method.
    /// [`RTCRtpSender.setStreams`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setStreams)
    pub fn set_streams(&self, streams: &MediaStream) -> Undefined {
        self.inner
            .call("setStreams", &[streams.into()])
            .as_::<Undefined>()
    }
}
impl RTCRtpSender {
    /// The getStats method.
    /// [`RTCRtpSender.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getStats)
    pub fn get_stats(&self) -> Promise<RTCStatsReport> {
        self.inner
            .call("getStats", &[])
            .as_::<Promise<RTCStatsReport>>()
    }
}
impl RTCRtpSender {
    /// Getter of the `transform` attribute.
    /// [`RTCRtpSender.transform`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transform)
    pub fn transform(&self) -> Any {
        self.inner.get("transform").as_::<Any>()
    }

    /// Setter of the `transform` attribute.
    /// [`RTCRtpSender.transform`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transform)
    pub fn set_transform(&mut self, value: &Any) {
        self.inner.set("transform", value);
    }
}
impl RTCRtpSender {
    /// The generateKeyFrame method.
    /// [`RTCRtpSender.generateKeyFrame`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/generateKeyFrame)
    pub fn generate_key_frame0(&self) -> Promise<Undefined> {
        self.inner
            .call("generateKeyFrame", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The generateKeyFrame method.
    /// [`RTCRtpSender.generateKeyFrame`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/generateKeyFrame)
    pub fn generate_key_frame1(&self, rids: &TypedArray<JsString>) -> Promise<Undefined> {
        self.inner
            .call("generateKeyFrame", &[rids.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl RTCRtpSender {
    /// Getter of the `dtmf` attribute.
    /// [`RTCRtpSender.dtmf`](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/dtmf)
    pub fn dtmf(&self) -> RTCDTMFSender {
        self.inner.get("dtmf").as_::<RTCDTMFSender>()
    }
}
