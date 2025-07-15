use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AttributionReportingRequestOptions {
    inner: emlite::Val,
}
impl FromVal for AttributionReportingRequestOptions {
    fn from_val(v: &emlite::Val) -> Self {
        AttributionReportingRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AttributionReportingRequestOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AttributionReportingRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AttributionReportingRequestOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AttributionReportingRequestOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AttributionReportingRequestOptions> for emlite::Val {
    fn from(s: AttributionReportingRequestOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AttributionReportingRequestOptions> for emlite::Val {
    fn from(s: &AttributionReportingRequestOptions) -> emlite::Val {
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
    inner: emlite::Val,
}
impl FromVal for PrivateToken {
    fn from_val(v: &emlite::Val) -> Self {
        PrivateToken { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PrivateToken {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PrivateToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PrivateToken {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PrivateToken {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PrivateToken> for emlite::Val {
    fn from(s: PrivateToken) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PrivateToken> for emlite::Val {
    fn from(s: &PrivateToken) -> emlite::Val {
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
    pub fn issuers(&self) -> Sequence<String> {
        self.inner.get("issuers").as_::<Sequence<String>>()
    }

    pub fn set_issuers(&mut self, value: &Sequence<String>) {
        self.inner.set("issuers", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XMLHttpRequest {
    inner: XMLHttpRequestEventTarget,
}
impl FromVal for XMLHttpRequest {
    fn from_val(v: &emlite::Val) -> Self {
        XMLHttpRequest {
            inner: XMLHttpRequestEventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for XMLHttpRequest {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XMLHttpRequest {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XMLHttpRequest> for emlite::Val {
    fn from(s: XMLHttpRequest) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XMLHttpRequest> for emlite::Val {
    fn from(s: &XMLHttpRequest) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XMLHttpRequest);

impl XMLHttpRequest {
    pub fn new() -> XMLHttpRequest {
        Self {
            inner: emlite::Val::global("XMLHttpRequest")
                .new(&[])
                .as_::<XMLHttpRequestEventTarget>(),
        }
    }
}
impl XMLHttpRequest {
    pub fn onreadystatechange(&self) -> Any {
        self.inner.get("onreadystatechange").as_::<Any>()
    }

    pub fn set_onreadystatechange(&mut self, value: &Any) {
        self.inner.set("onreadystatechange", value);
    }
}
impl XMLHttpRequest {
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl XMLHttpRequest {
    pub fn open0(&self, method: &str, url: &str, async_: bool) -> Undefined {
        self.inner
            .call("open", &[method.into(), url.into(), async_.into()])
            .as_::<Undefined>()
    }

    pub fn open1(&self, method: &str, url: &str, async_: bool, username: &str) -> Undefined {
        self.inner
            .call(
                "open",
                &[method.into(), url.into(), async_.into(), username.into()],
            )
            .as_::<Undefined>()
    }

    pub fn open2(
        &self,
        method: &str,
        url: &str,
        async_: bool,
        username: &str,
        password: &str,
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
    pub fn set_request_header(&self, name: &str, value: &str) -> Undefined {
        self.inner
            .call("setRequestHeader", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl XMLHttpRequest {
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
impl XMLHttpRequest {
    pub fn with_credentials(&self) -> bool {
        self.inner.get("withCredentials").as_::<bool>()
    }

    pub fn set_with_credentials(&mut self, value: bool) {
        self.inner.set("withCredentials", value);
    }
}
impl XMLHttpRequest {
    pub fn upload(&self) -> XMLHttpRequestUpload {
        self.inner.get("upload").as_::<XMLHttpRequestUpload>()
    }
}
impl XMLHttpRequest {
    pub fn send0(&self) -> Undefined {
        self.inner.call("send", &[]).as_::<Undefined>()
    }

    pub fn send1(&self, body: &Any) -> Undefined {
        self.inner.call("send", &[body.into()]).as_::<Undefined>()
    }
}
impl XMLHttpRequest {
    pub fn abort(&self) -> Undefined {
        self.inner.call("abort", &[]).as_::<Undefined>()
    }
}
impl XMLHttpRequest {
    pub fn response_url(&self) -> String {
        self.inner.get("responseURL").as_::<String>()
    }
}
impl XMLHttpRequest {
    pub fn status(&self) -> u16 {
        self.inner.get("status").as_::<u16>()
    }
}
impl XMLHttpRequest {
    pub fn status_text(&self) -> String {
        self.inner.get("statusText").as_::<String>()
    }
}
impl XMLHttpRequest {
    pub fn get_response_header(&self, name: &str) -> String {
        self.inner
            .call("getResponseHeader", &[name.into()])
            .as_::<String>()
    }
}
impl XMLHttpRequest {
    pub fn get_all_response_headers(&self) -> String {
        self.inner
            .call("getAllResponseHeaders", &[])
            .as_::<String>()
    }
}
impl XMLHttpRequest {
    pub fn override_mime_type(&self, mime: &str) -> Undefined {
        self.inner
            .call("overrideMimeType", &[mime.into()])
            .as_::<Undefined>()
    }
}
impl XMLHttpRequest {
    pub fn response_type(&self) -> XMLHttpRequestResponseType {
        self.inner
            .get("responseType")
            .as_::<XMLHttpRequestResponseType>()
    }

    pub fn set_response_type(&mut self, value: &XMLHttpRequestResponseType) {
        self.inner.set("responseType", value);
    }
}
impl XMLHttpRequest {
    pub fn response(&self) -> Any {
        self.inner.get("response").as_::<Any>()
    }
}
impl XMLHttpRequest {
    pub fn response_text(&self) -> String {
        self.inner.get("responseText").as_::<String>()
    }
}
impl XMLHttpRequest {
    pub fn response_xml(&self) -> Document {
        self.inner.get("responseXML").as_::<Document>()
    }
}
impl XMLHttpRequest {
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
    pub fn set_private_token(&self, private_token: &PrivateToken) -> Undefined {
        self.inner
            .call("setPrivateToken", &[private_token.into()])
            .as_::<Undefined>()
    }
}
