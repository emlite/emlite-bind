use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for Response {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Response {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Response> for emlite::Val {
    fn from(s: Response) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Response {
    pub fn new0() -> Response {
        Self {
            inner: emlite::Val::global("Response")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(body: jsbind::Any) -> Response {
        Self {
            inner: emlite::Val::global("Response")
                .new(&[body.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(body: jsbind::Any, init: jsbind::Any) -> Response {
        Self {
            inner: emlite::Val::global("Response")
                .new(&[body.into(), init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Response {
    pub fn error() -> Response {
        emlite::Val::global("response")
            .call("error", &[])
            .as_::<Response>()
    }
}
impl Response {
    pub fn redirect0(url: jsbind::USVString) -> Response {
        emlite::Val::global("response")
            .call("redirect", &[url.into()])
            .as_::<Response>()
    }

    pub fn redirect1(url: jsbind::USVString, status: u16) -> Response {
        emlite::Val::global("response")
            .call("redirect", &[url.into(), status.into()])
            .as_::<Response>()
    }
}
impl Response {
    pub fn json(&self) -> jsbind::Promise {
        self.inner.call("json", &[]).as_::<jsbind::Promise>()
    }
}
impl Response {
    pub fn type_(&self) -> ResponseType {
        self.inner.get("type").as_::<ResponseType>()
    }
}
impl Response {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
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
    pub fn status_text(&self) -> jsbind::ByteString {
        self.inner.get("statusText").as_::<jsbind::ByteString>()
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
    pub fn array_buffer(&self) -> jsbind::Promise {
        self.inner.call("arrayBuffer", &[]).as_::<jsbind::Promise>()
    }
}
impl Response {
    pub fn blob(&self) -> jsbind::Promise {
        self.inner.call("blob", &[]).as_::<jsbind::Promise>()
    }
}
impl Response {
    pub fn bytes(&self) -> jsbind::Promise {
        self.inner.call("bytes", &[]).as_::<jsbind::Promise>()
    }
}
impl Response {
    pub fn form_data(&self) -> jsbind::Promise {
        self.inner.call("formData", &[]).as_::<jsbind::Promise>()
    }
}
impl Response {
    pub fn text(&self) -> jsbind::Promise {
        self.inner.call("text", &[]).as_::<jsbind::Promise>()
    }
}
