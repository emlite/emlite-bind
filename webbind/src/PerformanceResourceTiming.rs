use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceResourceTiming {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceResourceTiming {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceResourceTiming {
            inner: PerformanceEntry::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceResourceTiming {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceResourceTiming {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceResourceTiming {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceResourceTiming {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PerformanceResourceTiming> for emlite::Val {
    fn from(s: PerformanceResourceTiming) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceResourceTiming);

impl PerformanceResourceTiming {
    pub fn initiator_type(&self) -> jsbind::DOMString {
        self.inner.get("initiatorType").as_::<jsbind::DOMString>()
    }
}
impl PerformanceResourceTiming {
    pub fn delivery_type(&self) -> jsbind::DOMString {
        self.inner.get("deliveryType").as_::<jsbind::DOMString>()
    }
}
impl PerformanceResourceTiming {
    pub fn next_hop_protocol(&self) -> jsbind::ByteString {
        self.inner
            .get("nextHopProtocol")
            .as_::<jsbind::ByteString>()
    }
}
impl PerformanceResourceTiming {
    pub fn worker_start(&self) -> jsbind::Any {
        self.inner.get("workerStart").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn redirect_start(&self) -> jsbind::Any {
        self.inner.get("redirectStart").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn redirect_end(&self) -> jsbind::Any {
        self.inner.get("redirectEnd").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn fetch_start(&self) -> jsbind::Any {
        self.inner.get("fetchStart").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn domain_lookup_start(&self) -> jsbind::Any {
        self.inner.get("domainLookupStart").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn domain_lookup_end(&self) -> jsbind::Any {
        self.inner.get("domainLookupEnd").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn connect_start(&self) -> jsbind::Any {
        self.inner.get("connectStart").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn connect_end(&self) -> jsbind::Any {
        self.inner.get("connectEnd").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn secure_connection_start(&self) -> jsbind::Any {
        self.inner.get("secureConnectionStart").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn request_start(&self) -> jsbind::Any {
        self.inner.get("requestStart").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn final_response_headers_start(&self) -> jsbind::Any {
        self.inner
            .get("finalResponseHeadersStart")
            .as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn first_interim_response_start(&self) -> jsbind::Any {
        self.inner
            .get("firstInterimResponseStart")
            .as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn response_start(&self) -> jsbind::Any {
        self.inner.get("responseStart").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn response_end(&self) -> jsbind::Any {
        self.inner.get("responseEnd").as_::<jsbind::Any>()
    }
}
impl PerformanceResourceTiming {
    pub fn transfer_size(&self) -> u64 {
        self.inner.get("transferSize").as_::<u64>()
    }
}
impl PerformanceResourceTiming {
    pub fn encoded_body_size(&self) -> u64 {
        self.inner.get("encodedBodySize").as_::<u64>()
    }
}
impl PerformanceResourceTiming {
    pub fn decoded_body_size(&self) -> u64 {
        self.inner.get("decodedBodySize").as_::<u64>()
    }
}
impl PerformanceResourceTiming {
    pub fn response_status(&self) -> u16 {
        self.inner.get("responseStatus").as_::<u16>()
    }
}
impl PerformanceResourceTiming {
    pub fn render_blocking_status(&self) -> RenderBlockingStatusType {
        self.inner
            .get("renderBlockingStatus")
            .as_::<RenderBlockingStatusType>()
    }
}
impl PerformanceResourceTiming {
    pub fn content_type(&self) -> jsbind::DOMString {
        self.inner.get("contentType").as_::<jsbind::DOMString>()
    }
}
impl PerformanceResourceTiming {
    pub fn content_encoding(&self) -> jsbind::DOMString {
        self.inner.get("contentEncoding").as_::<jsbind::DOMString>()
    }
}
impl PerformanceResourceTiming {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
impl PerformanceResourceTiming {
    pub fn server_timing(&self) -> jsbind::FrozenArray<PerformanceServerTiming> {
        self.inner
            .get("serverTiming")
            .as_::<jsbind::FrozenArray<PerformanceServerTiming>>()
    }
}
