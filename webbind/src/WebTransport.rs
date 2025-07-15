use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportConnectionStats {
    inner: emlite::Val,
}
impl FromVal for WebTransportConnectionStats {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportConnectionStats { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportConnectionStats {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportConnectionStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebTransportConnectionStats {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebTransportConnectionStats {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebTransportConnectionStats> for emlite::Val {
    fn from(s: WebTransportConnectionStats) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WebTransportConnectionStats> for emlite::Val {
    fn from(s: &WebTransportConnectionStats) -> emlite::Val {
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
    inner: emlite::Val,
}
impl FromVal for WebTransportCloseInfo {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportCloseInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportCloseInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportCloseInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebTransportCloseInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebTransportCloseInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebTransportCloseInfo> for emlite::Val {
    fn from(s: WebTransportCloseInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WebTransportCloseInfo> for emlite::Val {
    fn from(s: &WebTransportCloseInfo) -> emlite::Val {
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
    pub fn reason(&self) -> String {
        self.inner.get("reason").as_::<String>()
    }

    pub fn set_reason(&mut self, value: &str) {
        self.inner.set("reason", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportSendStreamOptions {
    inner: emlite::Val,
}
impl FromVal for WebTransportSendStreamOptions {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportSendStreamOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportSendStreamOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportSendStreamOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebTransportSendStreamOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebTransportSendStreamOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebTransportSendStreamOptions> for emlite::Val {
    fn from(s: WebTransportSendStreamOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WebTransportSendStreamOptions> for emlite::Val {
    fn from(s: &WebTransportSendStreamOptions) -> emlite::Val {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransport {
    inner: emlite::Val,
}
impl FromVal for WebTransport {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransport {
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
impl core::ops::Deref for WebTransport {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WebTransport {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WebTransport {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WebTransport> for emlite::Val {
    fn from(s: WebTransport) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WebTransport> for emlite::Val {
    fn from(s: &WebTransport) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebTransport);

impl WebTransport {
    pub fn new0(url: &str) -> WebTransport {
        Self {
            inner: emlite::Val::global("WebTransport")
                .new(&[url.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(url: &str, options: &Any) -> WebTransport {
        Self {
            inner: emlite::Val::global("WebTransport")
                .new(&[url.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl WebTransport {
    pub fn get_stats(&self) -> Promise {
        self.inner.call("getStats", &[]).as_::<Promise>()
    }
}
impl WebTransport {
    pub fn export_keying_material0(&self, label: &Any) -> Promise {
        self.inner
            .call("exportKeyingMaterial", &[label.into()])
            .as_::<Promise>()
    }

    pub fn export_keying_material1(&self, label: &Any, context: &Any) -> Promise {
        self.inner
            .call("exportKeyingMaterial", &[label.into(), context.into()])
            .as_::<Promise>()
    }
}
impl WebTransport {
    pub fn ready(&self) -> Promise {
        self.inner.get("ready").as_::<Promise>()
    }
}
impl WebTransport {
    pub fn reliability(&self) -> WebTransportReliabilityMode {
        self.inner
            .get("reliability")
            .as_::<WebTransportReliabilityMode>()
    }
}
impl WebTransport {
    pub fn congestion_control(&self) -> WebTransportCongestionControl {
        self.inner
            .get("congestionControl")
            .as_::<WebTransportCongestionControl>()
    }
}
impl WebTransport {
    pub fn anticipated_concurrent_incoming_unidirectional_streams(&self) -> u16 {
        self.inner
            .get("anticipatedConcurrentIncomingUnidirectionalStreams")
            .as_::<u16>()
    }

    pub fn set_anticipated_concurrent_incoming_unidirectional_streams(&mut self, value: u16) {
        self.inner
            .set("anticipatedConcurrentIncomingUnidirectionalStreams", value);
    }
}
impl WebTransport {
    pub fn anticipated_concurrent_incoming_bidirectional_streams(&self) -> u16 {
        self.inner
            .get("anticipatedConcurrentIncomingBidirectionalStreams")
            .as_::<u16>()
    }

    pub fn set_anticipated_concurrent_incoming_bidirectional_streams(&mut self, value: u16) {
        self.inner
            .set("anticipatedConcurrentIncomingBidirectionalStreams", value);
    }
}
impl WebTransport {
    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }
}
impl WebTransport {
    pub fn closed(&self) -> Promise {
        self.inner.get("closed").as_::<Promise>()
    }
}
impl WebTransport {
    pub fn draining(&self) -> Promise {
        self.inner.get("draining").as_::<Promise>()
    }
}
impl WebTransport {
    pub fn close0(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }

    pub fn close1(&self, close_info: &WebTransportCloseInfo) -> Undefined {
        self.inner
            .call("close", &[close_info.into()])
            .as_::<Undefined>()
    }
}
impl WebTransport {
    pub fn datagrams(&self) -> WebTransportDatagramDuplexStream {
        self.inner
            .get("datagrams")
            .as_::<WebTransportDatagramDuplexStream>()
    }
}
impl WebTransport {
    pub fn create_bidirectional_stream0(&self) -> Promise {
        self.inner
            .call("createBidirectionalStream", &[])
            .as_::<Promise>()
    }

    pub fn create_bidirectional_stream1(&self, options: &WebTransportSendStreamOptions) -> Promise {
        self.inner
            .call("createBidirectionalStream", &[options.into()])
            .as_::<Promise>()
    }
}
impl WebTransport {
    pub fn incoming_bidirectional_streams(&self) -> ReadableStream {
        self.inner
            .get("incomingBidirectionalStreams")
            .as_::<ReadableStream>()
    }
}
impl WebTransport {
    pub fn create_unidirectional_stream0(&self) -> Promise {
        self.inner
            .call("createUnidirectionalStream", &[])
            .as_::<Promise>()
    }

    pub fn create_unidirectional_stream1(
        &self,
        options: &WebTransportSendStreamOptions,
    ) -> Promise {
        self.inner
            .call("createUnidirectionalStream", &[options.into()])
            .as_::<Promise>()
    }
}
impl WebTransport {
    pub fn incoming_unidirectional_streams(&self) -> ReadableStream {
        self.inner
            .get("incomingUnidirectionalStreams")
            .as_::<ReadableStream>()
    }
}
impl WebTransport {
    pub fn create_send_group(&self) -> WebTransportSendGroup {
        self.inner
            .call("createSendGroup", &[])
            .as_::<WebTransportSendGroup>()
    }
}
impl WebTransport {
    pub fn supports_reliable_only() -> bool {
        emlite::Val::global("WebTransport")
            .get("supportsReliableOnly")
            .as_::<bool>()
    }
}
