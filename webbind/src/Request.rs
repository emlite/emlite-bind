use super::*;

/// The Request class.
/// [`Request`](https://developer.mozilla.org/en-US/docs/Web/API/Request)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Request {
    inner: Any,
}
impl FromVal for Request {
    fn from_val(v: &Any) -> Self {
        Request {
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
impl core::ops::Deref for Request {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Request {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Request {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Request {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Request> for Any {
    fn from(s: Request) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Request> for Any {
    fn from(s: &Request) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Request);

impl Request {
    /// The `new Request(..)` constructor, creating a new Request instance
    pub fn new0(input: &Any) -> Request {
        Self {
            inner: Any::global("Request").new(&[input.into()]).as_::<Any>(),
        }
    }

    /// The `new Request(..)` constructor, creating a new Request instance
    pub fn new1(input: &Any, init: &RequestInit) -> Request {
        Self {
            inner: Any::global("Request")
                .new(&[input.into(), init.into()])
                .as_::<Any>(),
        }
    }
}
impl Request {
    /// Getter of the `method` attribute.
    /// [`Request.method`](https://developer.mozilla.org/en-US/docs/Web/API/Request/method)
    pub fn method(&self) -> JsString {
        self.inner.get("method").as_::<JsString>()
    }
}
impl Request {
    /// Getter of the `url` attribute.
    /// [`Request.url`](https://developer.mozilla.org/en-US/docs/Web/API/Request/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}
impl Request {
    /// Getter of the `headers` attribute.
    /// [`Request.headers`](https://developer.mozilla.org/en-US/docs/Web/API/Request/headers)
    pub fn headers(&self) -> Headers {
        self.inner.get("headers").as_::<Headers>()
    }
}
impl Request {
    /// Getter of the `destination` attribute.
    /// [`Request.destination`](https://developer.mozilla.org/en-US/docs/Web/API/Request/destination)
    pub fn destination(&self) -> RequestDestination {
        self.inner.get("destination").as_::<RequestDestination>()
    }
}
impl Request {
    /// Getter of the `referrer` attribute.
    /// [`Request.referrer`](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer)
    pub fn referrer(&self) -> JsString {
        self.inner.get("referrer").as_::<JsString>()
    }
}
impl Request {
    /// Getter of the `referrerPolicy` attribute.
    /// [`Request.referrerPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy)
    pub fn referrer_policy(&self) -> ReferrerPolicy {
        self.inner.get("referrerPolicy").as_::<ReferrerPolicy>()
    }
}
impl Request {
    /// Getter of the `mode` attribute.
    /// [`Request.mode`](https://developer.mozilla.org/en-US/docs/Web/API/Request/mode)
    pub fn mode(&self) -> RequestMode {
        self.inner.get("mode").as_::<RequestMode>()
    }
}
impl Request {
    /// Getter of the `credentials` attribute.
    /// [`Request.credentials`](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)
    pub fn credentials(&self) -> RequestCredentials {
        self.inner.get("credentials").as_::<RequestCredentials>()
    }
}
impl Request {
    /// Getter of the `cache` attribute.
    /// [`Request.cache`](https://developer.mozilla.org/en-US/docs/Web/API/Request/cache)
    pub fn cache(&self) -> RequestCache {
        self.inner.get("cache").as_::<RequestCache>()
    }
}
impl Request {
    /// Getter of the `redirect` attribute.
    /// [`Request.redirect`](https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect)
    pub fn redirect(&self) -> RequestRedirect {
        self.inner.get("redirect").as_::<RequestRedirect>()
    }
}
impl Request {
    /// Getter of the `integrity` attribute.
    /// [`Request.integrity`](https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity)
    pub fn integrity(&self) -> JsString {
        self.inner.get("integrity").as_::<JsString>()
    }
}
impl Request {
    /// Getter of the `keepalive` attribute.
    /// [`Request.keepalive`](https://developer.mozilla.org/en-US/docs/Web/API/Request/keepalive)
    pub fn keepalive(&self) -> bool {
        self.inner.get("keepalive").as_::<bool>()
    }
}
impl Request {
    /// Getter of the `isReloadNavigation` attribute.
    /// [`Request.isReloadNavigation`](https://developer.mozilla.org/en-US/docs/Web/API/Request/isReloadNavigation)
    pub fn is_reload_navigation(&self) -> bool {
        self.inner.get("isReloadNavigation").as_::<bool>()
    }
}
impl Request {
    /// Getter of the `isHistoryNavigation` attribute.
    /// [`Request.isHistoryNavigation`](https://developer.mozilla.org/en-US/docs/Web/API/Request/isHistoryNavigation)
    pub fn is_history_navigation(&self) -> bool {
        self.inner.get("isHistoryNavigation").as_::<bool>()
    }
}
impl Request {
    /// Getter of the `signal` attribute.
    /// [`Request.signal`](https://developer.mozilla.org/en-US/docs/Web/API/Request/signal)
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }
}
impl Request {
    /// Getter of the `duplex` attribute.
    /// [`Request.duplex`](https://developer.mozilla.org/en-US/docs/Web/API/Request/duplex)
    pub fn duplex(&self) -> RequestDuplex {
        self.inner.get("duplex").as_::<RequestDuplex>()
    }
}
impl Request {
    /// The clone method.
    /// [`Request.clone`](https://developer.mozilla.org/en-US/docs/Web/API/Request/clone)
    pub fn clone_(&self) -> Request {
        self.inner.call("clone", &[]).as_::<Request>()
    }
}
impl Request {
    /// Getter of the `targetAddressSpace` attribute.
    /// [`Request.targetAddressSpace`](https://developer.mozilla.org/en-US/docs/Web/API/Request/targetAddressSpace)
    pub fn target_address_space(&self) -> IPAddressSpace {
        self.inner.get("targetAddressSpace").as_::<IPAddressSpace>()
    }
}
impl Request {
    /// Getter of the `body` attribute.
    /// [`Request.body`](https://developer.mozilla.org/en-US/docs/Web/API/Request/body)
    pub fn body(&self) -> ReadableStream {
        self.inner.get("body").as_::<ReadableStream>()
    }
}
impl Request {
    /// Getter of the `bodyUsed` attribute.
    /// [`Request.bodyUsed`](https://developer.mozilla.org/en-US/docs/Web/API/Request/bodyUsed)
    pub fn body_used(&self) -> bool {
        self.inner.get("bodyUsed").as_::<bool>()
    }
}
impl Request {
    /// The arrayBuffer method.
    /// [`Request.arrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/Request/arrayBuffer)
    pub fn array_buffer(&self) -> Promise<ArrayBuffer> {
        self.inner
            .call("arrayBuffer", &[])
            .as_::<Promise<ArrayBuffer>>()
    }
}
impl Request {
    /// The blob method.
    /// [`Request.blob`](https://developer.mozilla.org/en-US/docs/Web/API/Request/blob)
    pub fn blob(&self) -> Promise<Blob> {
        self.inner.call("blob", &[]).as_::<Promise<Blob>>()
    }
}
impl Request {
    /// The bytes method.
    /// [`Request.bytes`](https://developer.mozilla.org/en-US/docs/Web/API/Request/bytes)
    pub fn bytes(&self) -> Promise<Uint8Array> {
        self.inner.call("bytes", &[]).as_::<Promise<Uint8Array>>()
    }
}
impl Request {
    /// The formData method.
    /// [`Request.formData`](https://developer.mozilla.org/en-US/docs/Web/API/Request/formData)
    pub fn form_data(&self) -> Promise<FormData> {
        self.inner.call("formData", &[]).as_::<Promise<FormData>>()
    }
}
impl Request {
    /// The json method.
    /// [`Request.json`](https://developer.mozilla.org/en-US/docs/Web/API/Request/json)
    pub fn json(&self) -> Promise<Any> {
        self.inner.call("json", &[]).as_::<Promise<Any>>()
    }
}
impl Request {
    /// The text method.
    /// [`Request.text`](https://developer.mozilla.org/en-US/docs/Web/API/Request/text)
    pub fn text(&self) -> Promise<JsString> {
        self.inner.call("text", &[]).as_::<Promise<JsString>>()
    }
}
