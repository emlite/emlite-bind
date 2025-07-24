use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportConnectionStats {
    inner: Any,
}
impl FromVal for WebTransportConnectionStats {
    fn from_val(v: &Any) -> Self {
        WebTransportConnectionStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportConnectionStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportConnectionStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebTransportConnectionStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebTransportConnectionStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebTransportConnectionStats> for Any {
    fn from(s: WebTransportConnectionStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebTransportConnectionStats> for Any {
    fn from(s: &WebTransportConnectionStats) -> Any {
        s.inner.clone()
    }
}

impl WebTransportConnectionStats {
    pub fn bytes_sent(&self) -> u64 {
        self.inner.get("bytesSent").as_::<u64>()
    }

    pub fn set_bytes_sent(&mut self, value: u64) {
        self.inner.set("bytesSent", value);
    }
}
impl WebTransportConnectionStats {
    pub fn packets_sent(&self) -> u64 {
        self.inner.get("packetsSent").as_::<u64>()
    }

    pub fn set_packets_sent(&mut self, value: u64) {
        self.inner.set("packetsSent", value);
    }
}
impl WebTransportConnectionStats {
    pub fn bytes_lost(&self) -> u64 {
        self.inner.get("bytesLost").as_::<u64>()
    }

    pub fn set_bytes_lost(&mut self, value: u64) {
        self.inner.set("bytesLost", value);
    }
}
impl WebTransportConnectionStats {
    pub fn packets_lost(&self) -> u64 {
        self.inner.get("packetsLost").as_::<u64>()
    }

    pub fn set_packets_lost(&mut self, value: u64) {
        self.inner.set("packetsLost", value);
    }
}
impl WebTransportConnectionStats {
    pub fn bytes_received(&self) -> u64 {
        self.inner.get("bytesReceived").as_::<u64>()
    }

    pub fn set_bytes_received(&mut self, value: u64) {
        self.inner.set("bytesReceived", value);
    }
}
impl WebTransportConnectionStats {
    pub fn packets_received(&self) -> u64 {
        self.inner.get("packetsReceived").as_::<u64>()
    }

    pub fn set_packets_received(&mut self, value: u64) {
        self.inner.set("packetsReceived", value);
    }
}
impl WebTransportConnectionStats {
    pub fn smoothed_rtt(&self) -> Any {
        self.inner.get("smoothedRtt").as_::<Any>()
    }

    pub fn set_smoothed_rtt(&mut self, value: &Any) {
        self.inner.set("smoothedRtt", value);
    }
}
impl WebTransportConnectionStats {
    pub fn rtt_variation(&self) -> Any {
        self.inner.get("rttVariation").as_::<Any>()
    }

    pub fn set_rtt_variation(&mut self, value: &Any) {
        self.inner.set("rttVariation", value);
    }
}
impl WebTransportConnectionStats {
    pub fn min_rtt(&self) -> Any {
        self.inner.get("minRtt").as_::<Any>()
    }

    pub fn set_min_rtt(&mut self, value: &Any) {
        self.inner.set("minRtt", value);
    }
}
impl WebTransportConnectionStats {
    pub fn datagrams(&self) -> Any {
        self.inner.get("datagrams").as_::<Any>()
    }

    pub fn set_datagrams(&mut self, value: &Any) {
        self.inner.set("datagrams", value);
    }
}
impl WebTransportConnectionStats {
    pub fn estimated_send_rate(&self) -> u64 {
        self.inner.get("estimatedSendRate").as_::<u64>()
    }

    pub fn set_estimated_send_rate(&mut self, value: u64) {
        self.inner.set("estimatedSendRate", value);
    }
}
impl WebTransportConnectionStats {
    pub fn at_send_capacity(&self) -> bool {
        self.inner.get("atSendCapacity").as_::<bool>()
    }

    pub fn set_at_send_capacity(&mut self, value: bool) {
        self.inner.set("atSendCapacity", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportCloseInfo {
    inner: Any,
}
impl FromVal for WebTransportCloseInfo {
    fn from_val(v: &Any) -> Self {
        WebTransportCloseInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportCloseInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportCloseInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebTransportCloseInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebTransportCloseInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebTransportCloseInfo> for Any {
    fn from(s: WebTransportCloseInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebTransportCloseInfo> for Any {
    fn from(s: &WebTransportCloseInfo) -> Any {
        s.inner.clone()
    }
}

impl WebTransportCloseInfo {
    pub fn close_code(&self) -> u32 {
        self.inner.get("closeCode").as_::<u32>()
    }

    pub fn set_close_code(&mut self, value: u32) {
        self.inner.set("closeCode", value);
    }
}
impl WebTransportCloseInfo {
    pub fn reason(&self) -> USVString {
        self.inner.get("reason").as_::<USVString>()
    }

    pub fn set_reason(&mut self, value: &USVString) {
        self.inner.set("reason", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportSendStreamOptions {
    inner: Any,
}
impl FromVal for WebTransportSendStreamOptions {
    fn from_val(v: &Any) -> Self {
        WebTransportSendStreamOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportSendStreamOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportSendStreamOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebTransportSendStreamOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebTransportSendStreamOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebTransportSendStreamOptions> for Any {
    fn from(s: WebTransportSendStreamOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebTransportSendStreamOptions> for Any {
    fn from(s: &WebTransportSendStreamOptions) -> Any {
        s.inner.clone()
    }
}

impl WebTransportSendStreamOptions {
    pub fn wait_until_available(&self) -> bool {
        self.inner.get("waitUntilAvailable").as_::<bool>()
    }

    pub fn set_wait_until_available(&mut self, value: bool) {
        self.inner.set("waitUntilAvailable", value);
    }
}
/// The WebTransport class.
/// [`WebTransport`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransport {
    inner: Any,
}
impl FromVal for WebTransport {
    fn from_val(v: &Any) -> Self {
        WebTransport {
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
impl core::ops::Deref for WebTransport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebTransport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebTransport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebTransport> for Any {
    fn from(s: WebTransport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebTransport> for Any {
    fn from(s: &WebTransport) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebTransport);

impl WebTransport {
    /// The `new WebTransport(..)` constructor, creating a new WebTransport instance
    pub fn new0(url: &USVString) -> WebTransport {
        Self {
            inner: Any::global("WebTransport").new(&[url.into()]).as_::<Any>(),
        }
    }

    /// The `new WebTransport(..)` constructor, creating a new WebTransport instance
    pub fn new1(url: &USVString, options: &Any) -> WebTransport {
        Self {
            inner: Any::global("WebTransport")
                .new(&[url.into(), options.into()])
                .as_::<Any>(),
        }
    }
}
impl WebTransport {
    /// The getStats method.
    /// [`WebTransport.getStats`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/getStats)
    pub fn get_stats(&self) -> Promise<WebTransportConnectionStats> {
        self.inner
            .call("getStats", &[])
            .as_::<Promise<WebTransportConnectionStats>>()
    }
}
impl WebTransport {
    /// The exportKeyingMaterial method.
    /// [`WebTransport.exportKeyingMaterial`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/exportKeyingMaterial)
    pub fn export_keying_material0(&self, label: &Any) -> Promise<ArrayBuffer> {
        self.inner
            .call("exportKeyingMaterial", &[label.into()])
            .as_::<Promise<ArrayBuffer>>()
    }
    /// The exportKeyingMaterial method.
    /// [`WebTransport.exportKeyingMaterial`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/exportKeyingMaterial)
    pub fn export_keying_material1(&self, label: &Any, context: &Any) -> Promise<ArrayBuffer> {
        self.inner
            .call("exportKeyingMaterial", &[label.into(), context.into()])
            .as_::<Promise<ArrayBuffer>>()
    }
}
impl WebTransport {
    /// Getter of the `ready` attribute.
    /// [`WebTransport.ready`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/ready)
    pub fn ready(&self) -> Promise<Undefined> {
        self.inner.get("ready").as_::<Promise<Undefined>>()
    }
}
impl WebTransport {
    /// Getter of the `reliability` attribute.
    /// [`WebTransport.reliability`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/reliability)
    pub fn reliability(&self) -> WebTransportReliabilityMode {
        self.inner
            .get("reliability")
            .as_::<WebTransportReliabilityMode>()
    }
}
impl WebTransport {
    /// Getter of the `congestionControl` attribute.
    /// [`WebTransport.congestionControl`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/congestionControl)
    pub fn congestion_control(&self) -> WebTransportCongestionControl {
        self.inner
            .get("congestionControl")
            .as_::<WebTransportCongestionControl>()
    }
}
impl WebTransport {
    /// Getter of the `anticipatedConcurrentIncomingUnidirectionalStreams` attribute.
    /// [`WebTransport.anticipatedConcurrentIncomingUnidirectionalStreams`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/anticipatedConcurrentIncomingUnidirectionalStreams)
    pub fn anticipated_concurrent_incoming_unidirectional_streams(&self) -> u16 {
        self.inner
            .get("anticipatedConcurrentIncomingUnidirectionalStreams")
            .as_::<u16>()
    }

    /// Setter of the `anticipatedConcurrentIncomingUnidirectionalStreams` attribute.
    /// [`WebTransport.anticipatedConcurrentIncomingUnidirectionalStreams`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/anticipatedConcurrentIncomingUnidirectionalStreams)
    pub fn set_anticipated_concurrent_incoming_unidirectional_streams(&mut self, value: u16) {
        self.inner
            .set("anticipatedConcurrentIncomingUnidirectionalStreams", value);
    }
}
impl WebTransport {
    /// Getter of the `anticipatedConcurrentIncomingBidirectionalStreams` attribute.
    /// [`WebTransport.anticipatedConcurrentIncomingBidirectionalStreams`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/anticipatedConcurrentIncomingBidirectionalStreams)
    pub fn anticipated_concurrent_incoming_bidirectional_streams(&self) -> u16 {
        self.inner
            .get("anticipatedConcurrentIncomingBidirectionalStreams")
            .as_::<u16>()
    }

    /// Setter of the `anticipatedConcurrentIncomingBidirectionalStreams` attribute.
    /// [`WebTransport.anticipatedConcurrentIncomingBidirectionalStreams`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/anticipatedConcurrentIncomingBidirectionalStreams)
    pub fn set_anticipated_concurrent_incoming_bidirectional_streams(&mut self, value: u16) {
        self.inner
            .set("anticipatedConcurrentIncomingBidirectionalStreams", value);
    }
}
impl WebTransport {
    /// Getter of the `protocol` attribute.
    /// [`WebTransport.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/protocol)
    pub fn protocol(&self) -> DOMString {
        self.inner.get("protocol").as_::<DOMString>()
    }
}
impl WebTransport {
    /// Getter of the `closed` attribute.
    /// [`WebTransport.closed`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/closed)
    pub fn closed(&self) -> Promise<WebTransportCloseInfo> {
        self.inner
            .get("closed")
            .as_::<Promise<WebTransportCloseInfo>>()
    }
}
impl WebTransport {
    /// Getter of the `draining` attribute.
    /// [`WebTransport.draining`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/draining)
    pub fn draining(&self) -> Promise<Undefined> {
        self.inner.get("draining").as_::<Promise<Undefined>>()
    }
}
impl WebTransport {
    /// The close method.
    /// [`WebTransport.close`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/close)
    pub fn close0(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
    /// The close method.
    /// [`WebTransport.close`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/close)
    pub fn close1(&self, close_info: &WebTransportCloseInfo) -> Undefined {
        self.inner
            .call("close", &[close_info.into()])
            .as_::<Undefined>()
    }
}
impl WebTransport {
    /// Getter of the `datagrams` attribute.
    /// [`WebTransport.datagrams`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/datagrams)
    pub fn datagrams(&self) -> WebTransportDatagramDuplexStream {
        self.inner
            .get("datagrams")
            .as_::<WebTransportDatagramDuplexStream>()
    }
}
impl WebTransport {
    /// The createBidirectionalStream method.
    /// [`WebTransport.createBidirectionalStream`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createBidirectionalStream)
    pub fn create_bidirectional_stream0(&self) -> Promise<WebTransportBidirectionalStream> {
        self.inner
            .call("createBidirectionalStream", &[])
            .as_::<Promise<WebTransportBidirectionalStream>>()
    }
    /// The createBidirectionalStream method.
    /// [`WebTransport.createBidirectionalStream`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createBidirectionalStream)
    pub fn create_bidirectional_stream1(
        &self,
        options: &WebTransportSendStreamOptions,
    ) -> Promise<WebTransportBidirectionalStream> {
        self.inner
            .call("createBidirectionalStream", &[options.into()])
            .as_::<Promise<WebTransportBidirectionalStream>>()
    }
}
impl WebTransport {
    /// Getter of the `incomingBidirectionalStreams` attribute.
    /// [`WebTransport.incomingBidirectionalStreams`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/incomingBidirectionalStreams)
    pub fn incoming_bidirectional_streams(&self) -> ReadableStream {
        self.inner
            .get("incomingBidirectionalStreams")
            .as_::<ReadableStream>()
    }
}
impl WebTransport {
    /// The createUnidirectionalStream method.
    /// [`WebTransport.createUnidirectionalStream`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createUnidirectionalStream)
    pub fn create_unidirectional_stream0(&self) -> Promise<WebTransportSendStream> {
        self.inner
            .call("createUnidirectionalStream", &[])
            .as_::<Promise<WebTransportSendStream>>()
    }
    /// The createUnidirectionalStream method.
    /// [`WebTransport.createUnidirectionalStream`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createUnidirectionalStream)
    pub fn create_unidirectional_stream1(
        &self,
        options: &WebTransportSendStreamOptions,
    ) -> Promise<WebTransportSendStream> {
        self.inner
            .call("createUnidirectionalStream", &[options.into()])
            .as_::<Promise<WebTransportSendStream>>()
    }
}
impl WebTransport {
    /// Getter of the `incomingUnidirectionalStreams` attribute.
    /// [`WebTransport.incomingUnidirectionalStreams`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/incomingUnidirectionalStreams)
    pub fn incoming_unidirectional_streams(&self) -> ReadableStream {
        self.inner
            .get("incomingUnidirectionalStreams")
            .as_::<ReadableStream>()
    }
}
impl WebTransport {
    /// The createSendGroup method.
    /// [`WebTransport.createSendGroup`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/createSendGroup)
    pub fn create_send_group(&self) -> WebTransportSendGroup {
        self.inner
            .call("createSendGroup", &[])
            .as_::<WebTransportSendGroup>()
    }
}
impl WebTransport {
    /// Getter of the `supportsReliableOnly` static attribute.
    /// [`WebTransport.supportsReliableOnly`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransport/supportsReliableOnly)
    pub fn supports_reliable_only() -> bool {
        Any::global("WebTransport")
            .get("supportsReliableOnly")
            .as_::<bool>()
    }
}
