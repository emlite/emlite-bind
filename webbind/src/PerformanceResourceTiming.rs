use super::*;

/// The PerformanceResourceTiming class.
/// [`PerformanceResourceTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceResourceTiming {
    inner: PerformanceEntry,
}

impl FromVal for PerformanceResourceTiming {
    fn from_val(v: &Any) -> Self {
        PerformanceResourceTiming {
            inner: PerformanceEntry::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for PerformanceResourceTiming {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformanceResourceTiming {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PerformanceResourceTiming> for Any {
    fn from(s: PerformanceResourceTiming) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformanceResourceTiming> for Any {
    fn from(s: &PerformanceResourceTiming) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PerformanceResourceTiming);

impl PerformanceResourceTiming {
    /// Getter of the `initiatorType` attribute.
    /// [`PerformanceResourceTiming.initiatorType`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/initiatorType)
    pub fn initiator_type(&self) -> JsString {
        self.inner.get("initiatorType").as_::<JsString>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `deliveryType` attribute.
    /// [`PerformanceResourceTiming.deliveryType`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/deliveryType)
    pub fn delivery_type(&self) -> JsString {
        self.inner.get("deliveryType").as_::<JsString>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `nextHopProtocol` attribute.
    /// [`PerformanceResourceTiming.nextHopProtocol`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/nextHopProtocol)
    pub fn next_hop_protocol(&self) -> JsString {
        self.inner.get("nextHopProtocol").as_::<JsString>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `workerStart` attribute.
    /// [`PerformanceResourceTiming.workerStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/workerStart)
    pub fn worker_start(&self) -> Any {
        self.inner.get("workerStart").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `redirectStart` attribute.
    /// [`PerformanceResourceTiming.redirectStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/redirectStart)
    pub fn redirect_start(&self) -> Any {
        self.inner.get("redirectStart").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `redirectEnd` attribute.
    /// [`PerformanceResourceTiming.redirectEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/redirectEnd)
    pub fn redirect_end(&self) -> Any {
        self.inner.get("redirectEnd").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `fetchStart` attribute.
    /// [`PerformanceResourceTiming.fetchStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/fetchStart)
    pub fn fetch_start(&self) -> Any {
        self.inner.get("fetchStart").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `domainLookupStart` attribute.
    /// [`PerformanceResourceTiming.domainLookupStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/domainLookupStart)
    pub fn domain_lookup_start(&self) -> Any {
        self.inner.get("domainLookupStart").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `domainLookupEnd` attribute.
    /// [`PerformanceResourceTiming.domainLookupEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/domainLookupEnd)
    pub fn domain_lookup_end(&self) -> Any {
        self.inner.get("domainLookupEnd").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `connectStart` attribute.
    /// [`PerformanceResourceTiming.connectStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/connectStart)
    pub fn connect_start(&self) -> Any {
        self.inner.get("connectStart").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `connectEnd` attribute.
    /// [`PerformanceResourceTiming.connectEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/connectEnd)
    pub fn connect_end(&self) -> Any {
        self.inner.get("connectEnd").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `secureConnectionStart` attribute.
    /// [`PerformanceResourceTiming.secureConnectionStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/secureConnectionStart)
    pub fn secure_connection_start(&self) -> Any {
        self.inner.get("secureConnectionStart").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `requestStart` attribute.
    /// [`PerformanceResourceTiming.requestStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/requestStart)
    pub fn request_start(&self) -> Any {
        self.inner.get("requestStart").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `finalResponseHeadersStart` attribute.
    /// [`PerformanceResourceTiming.finalResponseHeadersStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/finalResponseHeadersStart)
    pub fn final_response_headers_start(&self) -> Any {
        self.inner.get("finalResponseHeadersStart").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `firstInterimResponseStart` attribute.
    /// [`PerformanceResourceTiming.firstInterimResponseStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/firstInterimResponseStart)
    pub fn first_interim_response_start(&self) -> Any {
        self.inner.get("firstInterimResponseStart").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `responseStart` attribute.
    /// [`PerformanceResourceTiming.responseStart`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseStart)
    pub fn response_start(&self) -> Any {
        self.inner.get("responseStart").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `responseEnd` attribute.
    /// [`PerformanceResourceTiming.responseEnd`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseEnd)
    pub fn response_end(&self) -> Any {
        self.inner.get("responseEnd").as_::<Any>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `transferSize` attribute.
    /// [`PerformanceResourceTiming.transferSize`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/transferSize)
    pub fn transfer_size(&self) -> u64 {
        self.inner.get("transferSize").as_::<u64>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `encodedBodySize` attribute.
    /// [`PerformanceResourceTiming.encodedBodySize`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/encodedBodySize)
    pub fn encoded_body_size(&self) -> u64 {
        self.inner.get("encodedBodySize").as_::<u64>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `decodedBodySize` attribute.
    /// [`PerformanceResourceTiming.decodedBodySize`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/decodedBodySize)
    pub fn decoded_body_size(&self) -> u64 {
        self.inner.get("decodedBodySize").as_::<u64>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `responseStatus` attribute.
    /// [`PerformanceResourceTiming.responseStatus`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseStatus)
    pub fn response_status(&self) -> u16 {
        self.inner.get("responseStatus").as_::<u16>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `renderBlockingStatus` attribute.
    /// [`PerformanceResourceTiming.renderBlockingStatus`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/renderBlockingStatus)
    pub fn render_blocking_status(&self) -> RenderBlockingStatusType {
        self.inner
            .get("renderBlockingStatus")
            .as_::<RenderBlockingStatusType>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `contentType` attribute.
    /// [`PerformanceResourceTiming.contentType`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/contentType)
    pub fn content_type(&self) -> JsString {
        self.inner.get("contentType").as_::<JsString>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `contentEncoding` attribute.
    /// [`PerformanceResourceTiming.contentEncoding`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/contentEncoding)
    pub fn content_encoding(&self) -> JsString {
        self.inner.get("contentEncoding").as_::<JsString>()
    }
}
impl PerformanceResourceTiming {
    /// Getter of the `serverTiming` attribute.
    /// [`PerformanceResourceTiming.serverTiming`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/serverTiming)
    pub fn server_timing(&self) -> TypedArray<PerformanceServerTiming> {
        self.inner
            .get("serverTiming")
            .as_::<TypedArray<PerformanceServerTiming>>()
    }
}
impl PerformanceResourceTiming {
    /// The toJSON method.
    /// [`PerformanceResourceTiming.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
