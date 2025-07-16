use super::*;

/// The Response class.
/// [`Response`](https://developer.mozilla.org/en-US/docs/Web/API/Response)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Response {
    inner: Any,
}
impl FromVal for Response {
    fn from_val(v: &Any) -> Self {
        Response {
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
impl core::ops::Deref for Response {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Response {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Response {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Response {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Response> for Any {
    fn from(s: Response) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Response> for Any {
    fn from(s: &Response) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Response);

impl Response {
    /// The `new Response(..)` constructor, creating a new Response instance
    pub fn new0() -> Response {
        Self {
            inner: Any::global("Response").new(&[]).as_::<Any>(),
        }
    }

    /// The `new Response(..)` constructor, creating a new Response instance
    pub fn new1(body: &Any) -> Response {
        Self {
            inner: Any::global("Response").new(&[body.into()]).as_::<Any>(),
        }
    }

    /// The `new Response(..)` constructor, creating a new Response instance
    pub fn new2(body: &Any, init: &Any) -> Response {
        Self {
            inner: Any::global("Response")
                .new(&[body.into(), init.into()])
                .as_::<Any>(),
        }
    }
}
impl Response {
    /// The error method.
    /// [`Response.error`](https://developer.mozilla.org/en-US/docs/Web/API/Response/error)
    pub fn error() -> Response {
        Any::global("Response").call("error", &[]).as_::<Response>()
    }
}
impl Response {
    /// The redirect method.
    /// [`Response.redirect`](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect)
    pub fn redirect0(url: &str) -> Response {
        Any::global("Response")
            .call("redirect", &[url.into()])
            .as_::<Response>()
    }
    /// The redirect method.
    /// [`Response.redirect`](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect)
    pub fn redirect1(url: &str, status: u16) -> Response {
        Any::global("Response")
            .call("redirect", &[url.into(), status.into()])
            .as_::<Response>()
    }
}
impl Response {
    /// The json method.
    /// [`Response.json`](https://developer.mozilla.org/en-US/docs/Web/API/Response/json)
    pub fn json(&self) -> Promise {
        self.inner.call("json", &[]).as_::<Promise>()
    }
}
impl Response {
    /// Getter of the `type` attribute.
    /// [`Response.type`](https://developer.mozilla.org/en-US/docs/Web/API/Response/type)
    pub fn type_(&self) -> ResponseType {
        self.inner.get("type").as_::<ResponseType>()
    }
}
impl Response {
    /// Getter of the `url` attribute.
    /// [`Response.url`](https://developer.mozilla.org/en-US/docs/Web/API/Response/url)
    pub fn url(&self) -> String {
        self.inner.get("url").as_::<String>()
    }
}
impl Response {
    /// Getter of the `redirected` attribute.
    /// [`Response.redirected`](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirected)
    pub fn redirected(&self) -> bool {
        self.inner.get("redirected").as_::<bool>()
    }
}
impl Response {
    /// Getter of the `status` attribute.
    /// [`Response.status`](https://developer.mozilla.org/en-US/docs/Web/API/Response/status)
    pub fn status(&self) -> u16 {
        self.inner.get("status").as_::<u16>()
    }
}
impl Response {
    /// Getter of the `ok` attribute.
    /// [`Response.ok`](https://developer.mozilla.org/en-US/docs/Web/API/Response/ok)
    pub fn ok(&self) -> bool {
        self.inner.get("ok").as_::<bool>()
    }
}
impl Response {
    /// Getter of the `statusText` attribute.
    /// [`Response.statusText`](https://developer.mozilla.org/en-US/docs/Web/API/Response/statusText)
    pub fn status_text(&self) -> String {
        self.inner.get("statusText").as_::<String>()
    }
}
impl Response {
    /// Getter of the `headers` attribute.
    /// [`Response.headers`](https://developer.mozilla.org/en-US/docs/Web/API/Response/headers)
    pub fn headers(&self) -> Headers {
        self.inner.get("headers").as_::<Headers>()
    }
}
impl Response {
    /// The clone method.
    /// [`Response.clone`](https://developer.mozilla.org/en-US/docs/Web/API/Response/clone)
    pub fn clone_(&self) -> Response {
        self.inner.call("clone", &[]).as_::<Response>()
    }
}
impl Response {
    /// Getter of the `body` attribute.
    /// [`Response.body`](https://developer.mozilla.org/en-US/docs/Web/API/Response/body)
    pub fn body(&self) -> ReadableStream {
        self.inner.get("body").as_::<ReadableStream>()
    }
}
impl Response {
    /// Getter of the `bodyUsed` attribute.
    /// [`Response.bodyUsed`](https://developer.mozilla.org/en-US/docs/Web/API/Response/bodyUsed)
    pub fn body_used(&self) -> bool {
        self.inner.get("bodyUsed").as_::<bool>()
    }
}
impl Response {
    /// The arrayBuffer method.
    /// [`Response.arrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer)
    pub fn array_buffer(&self) -> Promise {
        self.inner.call("arrayBuffer", &[]).as_::<Promise>()
    }
}
impl Response {
    /// The blob method.
    /// [`Response.blob`](https://developer.mozilla.org/en-US/docs/Web/API/Response/blob)
    pub fn blob(&self) -> Promise {
        self.inner.call("blob", &[]).as_::<Promise>()
    }
}
impl Response {
    /// The bytes method.
    /// [`Response.bytes`](https://developer.mozilla.org/en-US/docs/Web/API/Response/bytes)
    pub fn bytes(&self) -> Promise {
        self.inner.call("bytes", &[]).as_::<Promise>()
    }
}
impl Response {
    /// The formData method.
    /// [`Response.formData`](https://developer.mozilla.org/en-US/docs/Web/API/Response/formData)
    pub fn form_data(&self) -> Promise {
        self.inner.call("formData", &[]).as_::<Promise>()
    }
}
impl Response {
    /// The text method.
    /// [`Response.text`](https://developer.mozilla.org/en-US/docs/Web/API/Response/text)
    pub fn text(&self) -> Promise {
        self.inner.call("text", &[]).as_::<Promise>()
    }
}
