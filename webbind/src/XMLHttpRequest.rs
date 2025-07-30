use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionReportingRequestOptions {
    inner: Any,
}
impl FromVal for AttributionReportingRequestOptions {
    fn from_val(v: &Any) -> Self {
        AttributionReportingRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AttributionReportingRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionReportingRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AttributionReportingRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AttributionReportingRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AttributionReportingRequestOptions> for Any {
    fn from(s: AttributionReportingRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AttributionReportingRequestOptions> for Any {
    fn from(s: &AttributionReportingRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl AttributionReportingRequestOptions {
    pub fn event_source_eligible(&self) -> bool {
        self.inner.get("eventSourceEligible").as_::<bool>()
    }

    pub fn set_event_source_eligible(&mut self, value: bool) {
        self.inner.set("eventSourceEligible", value);
    }
}
impl AttributionReportingRequestOptions {
    pub fn trigger_eligible(&self) -> bool {
        self.inner.get("triggerEligible").as_::<bool>()
    }

    pub fn set_trigger_eligible(&mut self, value: bool) {
        self.inner.set("triggerEligible", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PrivateToken {
    inner: Any,
}
impl FromVal for PrivateToken {
    fn from_val(v: &Any) -> Self {
        PrivateToken { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PrivateToken {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PrivateToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PrivateToken {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PrivateToken {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PrivateToken> for Any {
    fn from(s: PrivateToken) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PrivateToken> for Any {
    fn from(s: &PrivateToken) -> Any {
        s.inner.clone()
    }
}

impl PrivateToken {
    pub fn version(&self) -> TokenVersion {
        self.inner.get("version").as_::<TokenVersion>()
    }

    pub fn set_version(&mut self, value: &TokenVersion) {
        self.inner.set("version", value);
    }
}
impl PrivateToken {
    pub fn operation(&self) -> OperationType {
        self.inner.get("operation").as_::<OperationType>()
    }

    pub fn set_operation(&mut self, value: &OperationType) {
        self.inner.set("operation", value);
    }
}
impl PrivateToken {
    pub fn refresh_policy(&self) -> RefreshPolicy {
        self.inner.get("refreshPolicy").as_::<RefreshPolicy>()
    }

    pub fn set_refresh_policy(&mut self, value: &RefreshPolicy) {
        self.inner.set("refreshPolicy", value);
    }
}
impl PrivateToken {
    pub fn issuers(&self) -> TypedArray<JsString> {
        self.inner.get("issuers").as_::<TypedArray<JsString>>()
    }

    pub fn set_issuers(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("issuers", value);
    }
}
/// The XMLHttpRequest class.
/// [`XMLHttpRequest`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XMLHttpRequest {
    inner: XMLHttpRequestEventTarget,
}
impl FromVal for XMLHttpRequest {
    fn from_val(v: &Any) -> Self {
        XMLHttpRequest {
            inner: XMLHttpRequestEventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XMLHttpRequest {
    type Target = XMLHttpRequestEventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XMLHttpRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XMLHttpRequest {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XMLHttpRequest {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XMLHttpRequest> for Any {
    fn from(s: XMLHttpRequest) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XMLHttpRequest> for Any {
    fn from(s: &XMLHttpRequest) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XMLHttpRequest);

impl XMLHttpRequest {
    /// The `new XMLHttpRequest(..)` constructor, creating a new XMLHttpRequest instance
    pub fn new() -> XMLHttpRequest {
        Self {
            inner: Any::global("XMLHttpRequest")
                .new(&[])
                .as_::<XMLHttpRequestEventTarget>(),
        }
    }
}
impl XMLHttpRequest {
    /// Getter of the `onreadystatechange` attribute.
    /// [`XMLHttpRequest.onreadystatechange`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/onreadystatechange)
    pub fn onreadystatechange(&self) -> Any {
        self.inner.get("onreadystatechange").as_::<Any>()
    }

    /// Setter of the `onreadystatechange` attribute.
    /// [`XMLHttpRequest.onreadystatechange`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/onreadystatechange)
    pub fn set_onreadystatechange(&mut self, value: &Any) {
        self.inner.set("onreadystatechange", value);
    }
}
impl XMLHttpRequest {
    /// Getter of the `readyState` attribute.
    /// [`XMLHttpRequest.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/readyState)
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl XMLHttpRequest {
    /// The open method.
    /// [`XMLHttpRequest.open`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)
    pub fn open0(&self, method: &JsString, url: &JsString, async_: bool) -> Undefined {
        self.inner
            .call("open", &[method.into(), url.into(), async_.into()])
            .as_::<Undefined>()
    }
    /// The open method.
    /// [`XMLHttpRequest.open`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)
    pub fn open1(
        &self,
        method: &JsString,
        url: &JsString,
        async_: bool,
        username: &JsString,
    ) -> Undefined {
        self.inner
            .call(
                "open",
                &[method.into(), url.into(), async_.into(), username.into()],
            )
            .as_::<Undefined>()
    }
    /// The open method.
    /// [`XMLHttpRequest.open`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)
    pub fn open2(
        &self,
        method: &JsString,
        url: &JsString,
        async_: bool,
        username: &JsString,
        password: &JsString,
    ) -> Undefined {
        self.inner
            .call(
                "open",
                &[
                    method.into(),
                    url.into(),
                    async_.into(),
                    username.into(),
                    password.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl XMLHttpRequest {
    /// The setRequestHeader method.
    /// [`XMLHttpRequest.setRequestHeader`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/setRequestHeader)
    pub fn set_request_header(&self, name: &JsString, value: &JsString) -> Undefined {
        self.inner
            .call("setRequestHeader", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl XMLHttpRequest {
    /// Getter of the `timeout` attribute.
    /// [`XMLHttpRequest.timeout`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/timeout)
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    /// Setter of the `timeout` attribute.
    /// [`XMLHttpRequest.timeout`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/timeout)
    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl XMLHttpRequest {
    /// Getter of the `withCredentials` attribute.
    /// [`XMLHttpRequest.withCredentials`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/withCredentials)
    pub fn with_credentials(&self) -> bool {
        self.inner.get("withCredentials").as_::<bool>()
    }

    /// Setter of the `withCredentials` attribute.
    /// [`XMLHttpRequest.withCredentials`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/withCredentials)
    pub fn set_with_credentials(&mut self, value: bool) {
        self.inner.set("withCredentials", value);
    }
}
impl XMLHttpRequest {
    /// Getter of the `upload` attribute.
    /// [`XMLHttpRequest.upload`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/upload)
    pub fn upload(&self) -> XMLHttpRequestUpload {
        self.inner.get("upload").as_::<XMLHttpRequestUpload>()
    }
}
impl XMLHttpRequest {
    /// The send method.
    /// [`XMLHttpRequest.send`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    pub fn send0(&self) -> Undefined {
        self.inner.call("send", &[]).as_::<Undefined>()
    }
    /// The send method.
    /// [`XMLHttpRequest.send`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)
    pub fn send1(&self, body: &Any) -> Undefined {
        self.inner.call("send", &[body.into()]).as_::<Undefined>()
    }
}
impl XMLHttpRequest {
    /// The abort method.
    /// [`XMLHttpRequest.abort`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/abort)
    pub fn abort(&self) -> Undefined {
        self.inner.call("abort", &[]).as_::<Undefined>()
    }
}
impl XMLHttpRequest {
    /// Getter of the `responseURL` attribute.
    /// [`XMLHttpRequest.responseURL`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseURL)
    pub fn response_url(&self) -> JsString {
        self.inner.get("responseURL").as_::<JsString>()
    }
}
impl XMLHttpRequest {
    /// Getter of the `status` attribute.
    /// [`XMLHttpRequest.status`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/status)
    pub fn status(&self) -> u16 {
        self.inner.get("status").as_::<u16>()
    }
}
impl XMLHttpRequest {
    /// Getter of the `statusText` attribute.
    /// [`XMLHttpRequest.statusText`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/statusText)
    pub fn status_text(&self) -> JsString {
        self.inner.get("statusText").as_::<JsString>()
    }
}
impl XMLHttpRequest {
    /// The getResponseHeader method.
    /// [`XMLHttpRequest.getResponseHeader`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/getResponseHeader)
    pub fn get_response_header(&self, name: &JsString) -> JsString {
        self.inner
            .call("getResponseHeader", &[name.into()])
            .as_::<JsString>()
    }
}
impl XMLHttpRequest {
    /// The getAllResponseHeaders method.
    /// [`XMLHttpRequest.getAllResponseHeaders`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/getAllResponseHeaders)
    pub fn get_all_response_headers(&self) -> JsString {
        self.inner
            .call("getAllResponseHeaders", &[])
            .as_::<JsString>()
    }
}
impl XMLHttpRequest {
    /// The overrideMimeType method.
    /// [`XMLHttpRequest.overrideMimeType`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/overrideMimeType)
    pub fn override_mime_type(&self, mime: &JsString) -> Undefined {
        self.inner
            .call("overrideMimeType", &[mime.into()])
            .as_::<Undefined>()
    }
}
impl XMLHttpRequest {
    /// Getter of the `responseType` attribute.
    /// [`XMLHttpRequest.responseType`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)
    pub fn response_type(&self) -> XMLHttpRequestResponseType {
        self.inner
            .get("responseType")
            .as_::<XMLHttpRequestResponseType>()
    }

    /// Setter of the `responseType` attribute.
    /// [`XMLHttpRequest.responseType`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)
    pub fn set_response_type(&mut self, value: &XMLHttpRequestResponseType) {
        self.inner.set("responseType", value);
    }
}
impl XMLHttpRequest {
    /// Getter of the `response` attribute.
    /// [`XMLHttpRequest.response`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/response)
    pub fn response(&self) -> Any {
        self.inner.get("response").as_::<Any>()
    }
}
impl XMLHttpRequest {
    /// Getter of the `responseText` attribute.
    /// [`XMLHttpRequest.responseText`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseText)
    pub fn response_text(&self) -> JsString {
        self.inner.get("responseText").as_::<JsString>()
    }
}
impl XMLHttpRequest {
    /// Getter of the `responseXML` attribute.
    /// [`XMLHttpRequest.responseXML`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseXML)
    pub fn response_xml(&self) -> Document {
        self.inner.get("responseXML").as_::<Document>()
    }
}
impl XMLHttpRequest {
    /// The setAttributionReporting method.
    /// [`XMLHttpRequest.setAttributionReporting`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/setAttributionReporting)
    pub fn set_attribution_reporting(
        &self,
        options: &AttributionReportingRequestOptions,
    ) -> Undefined {
        self.inner
            .call("setAttributionReporting", &[options.into()])
            .as_::<Undefined>()
    }
}
impl XMLHttpRequest {
    /// The setPrivateToken method.
    /// [`XMLHttpRequest.setPrivateToken`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/setPrivateToken)
    pub fn set_private_token(&self, private_token: &PrivateToken) -> Undefined {
        self.inner
            .call("setPrivateToken", &[private_token.into()])
            .as_::<Undefined>()
    }
}
