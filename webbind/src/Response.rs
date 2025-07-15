use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Response {
    inner: emlite::Val,
}
impl FromVal for Response {
    fn from_val(v: &emlite::Val) -> Self {
        Response {
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
impl core::ops::Deref for Response {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Response {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Response {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Response {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Response> for emlite::Val {
    fn from(s: Response) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Response> for emlite::Val {
    fn from(s: &Response) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Response);

impl Response {
    pub fn new0() -> Response {
        Self {
            inner: emlite::Val::global("Response")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(body: Any) -> Response {
        Self {
            inner: emlite::Val::global("Response")
                .new(&[body.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(body: Any, init: Any) -> Response {
        Self {
            inner: emlite::Val::global("Response")
                .new(&[body.into(), init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Response {
    pub fn error() -> Response {
        emlite::Val::global("Response")
            .call("error", &[])
            .as_::<Response>()
    }
}
impl Response {
    pub fn redirect0(url: USVString) -> Response {
        emlite::Val::global("Response")
            .call("redirect", &[url.into()])
            .as_::<Response>()
    }

    pub fn redirect1(url: USVString, status: u16) -> Response {
        emlite::Val::global("Response")
            .call("redirect", &[url.into(), status.into()])
            .as_::<Response>()
    }
}
impl Response {
    pub fn json(&self) -> Promise {
        self.inner.call("json", &[]).as_::<Promise>()
    }
}
impl Response {
    pub fn type_(&self) -> ResponseType {
        self.inner.get("type").as_::<ResponseType>()
    }
}
impl Response {
    pub fn url(&self) -> USVString {
        self.inner.get("url").as_::<USVString>()
    }
}
impl Response {
    pub fn redirected(&self) -> bool {
        self.inner.get("redirected").as_::<bool>()
    }
}
impl Response {
    pub fn status(&self) -> u16 {
        self.inner.get("status").as_::<u16>()
    }
}
impl Response {
    pub fn ok(&self) -> bool {
        self.inner.get("ok").as_::<bool>()
    }
}
impl Response {
    pub fn status_text(&self) -> ByteString {
        self.inner.get("statusText").as_::<ByteString>()
    }
}
impl Response {
    pub fn headers(&self) -> Headers {
        self.inner.get("headers").as_::<Headers>()
    }
}
impl Response {
    pub fn clone_(&self) -> Response {
        self.inner.call("clone", &[]).as_::<Response>()
    }
}
impl Response {
    pub fn body(&self) -> ReadableStream {
        self.inner.get("body").as_::<ReadableStream>()
    }
}
impl Response {
    pub fn body_used(&self) -> bool {
        self.inner.get("bodyUsed").as_::<bool>()
    }
}
impl Response {
    pub fn array_buffer(&self) -> Promise {
        self.inner.call("arrayBuffer", &[]).as_::<Promise>()
    }
}
impl Response {
    pub fn blob(&self) -> Promise {
        self.inner.call("blob", &[]).as_::<Promise>()
    }
}
impl Response {
    pub fn bytes(&self) -> Promise {
        self.inner.call("bytes", &[]).as_::<Promise>()
    }
}
impl Response {
    pub fn form_data(&self) -> Promise {
        self.inner.call("formData", &[]).as_::<Promise>()
    }
}
impl Response {
    pub fn text(&self) -> Promise {
        self.inner.call("text", &[]).as_::<Promise>()
    }
}
