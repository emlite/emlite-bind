use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Request {
    inner: emlite::Val,
}
impl FromVal for Request {
    fn from_val(v: &emlite::Val) -> Self {
        Request {
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
impl core::ops::Deref for Request {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Request {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Request {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Request {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Request> for emlite::Val {
    fn from(s: Request) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Request);

impl Request {
    pub fn new0(input: Any) -> Request {
        Self {
            inner: emlite::Val::global("Request")
                .new(&[input.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(input: Any, init: Any) -> Request {
        Self {
            inner: emlite::Val::global("Request")
                .new(&[input.into(), init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Request {
    pub fn method(&self) -> ByteString {
        self.inner.get("method").as_::<ByteString>()
    }
}
impl Request {
    pub fn url(&self) -> USVString {
        self.inner.get("url").as_::<USVString>()
    }
}
impl Request {
    pub fn headers(&self) -> Headers {
        self.inner.get("headers").as_::<Headers>()
    }
}
impl Request {
    pub fn destination(&self) -> RequestDestination {
        self.inner.get("destination").as_::<RequestDestination>()
    }
}
impl Request {
    pub fn referrer(&self) -> USVString {
        self.inner.get("referrer").as_::<USVString>()
    }
}
impl Request {
    pub fn referrer_policy(&self) -> ReferrerPolicy {
        self.inner.get("referrerPolicy").as_::<ReferrerPolicy>()
    }
}
impl Request {
    pub fn mode(&self) -> RequestMode {
        self.inner.get("mode").as_::<RequestMode>()
    }
}
impl Request {
    pub fn credentials(&self) -> RequestCredentials {
        self.inner.get("credentials").as_::<RequestCredentials>()
    }
}
impl Request {
    pub fn cache(&self) -> RequestCache {
        self.inner.get("cache").as_::<RequestCache>()
    }
}
impl Request {
    pub fn redirect(&self) -> RequestRedirect {
        self.inner.get("redirect").as_::<RequestRedirect>()
    }
}
impl Request {
    pub fn integrity(&self) -> DOMString {
        self.inner.get("integrity").as_::<DOMString>()
    }
}
impl Request {
    pub fn keepalive(&self) -> bool {
        self.inner.get("keepalive").as_::<bool>()
    }
}
impl Request {
    pub fn is_reload_navigation(&self) -> bool {
        self.inner.get("isReloadNavigation").as_::<bool>()
    }
}
impl Request {
    pub fn is_history_navigation(&self) -> bool {
        self.inner.get("isHistoryNavigation").as_::<bool>()
    }
}
impl Request {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }
}
impl Request {
    pub fn duplex(&self) -> RequestDuplex {
        self.inner.get("duplex").as_::<RequestDuplex>()
    }
}
impl Request {
    pub fn clone_(&self) -> Request {
        self.inner.call("clone", &[]).as_::<Request>()
    }
}
impl Request {
    pub fn target_address_space(&self) -> IPAddressSpace {
        self.inner.get("targetAddressSpace").as_::<IPAddressSpace>()
    }
}
impl Request {
    pub fn body(&self) -> ReadableStream {
        self.inner.get("body").as_::<ReadableStream>()
    }
}
impl Request {
    pub fn body_used(&self) -> bool {
        self.inner.get("bodyUsed").as_::<bool>()
    }
}
impl Request {
    pub fn array_buffer(&self) -> Promise {
        self.inner.call("arrayBuffer", &[]).as_::<Promise>()
    }
}
impl Request {
    pub fn blob(&self) -> Promise {
        self.inner.call("blob", &[]).as_::<Promise>()
    }
}
impl Request {
    pub fn bytes(&self) -> Promise {
        self.inner.call("bytes", &[]).as_::<Promise>()
    }
}
impl Request {
    pub fn form_data(&self) -> Promise {
        self.inner.call("formData", &[]).as_::<Promise>()
    }
}
impl Request {
    pub fn json(&self) -> Promise {
        self.inner.call("json", &[]).as_::<Promise>()
    }
}
impl Request {
    pub fn text(&self) -> Promise {
        self.inner.call("text", &[]).as_::<Promise>()
    }
}
