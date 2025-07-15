use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLAreaElement {
    inner: HTMLElement,
}
impl FromVal for HTMLAreaElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLAreaElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLAreaElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLAreaElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLAreaElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLAreaElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLAreaElement> for emlite::Val {
    fn from(s: HTMLAreaElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLAreaElement);

impl HTMLAreaElement {
    pub fn new() -> HTMLAreaElement {
        Self {
            inner: emlite::Val::global("HTMLAreaElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLAreaElement {
    pub fn alt(&self) -> DOMString {
        self.inner.get("alt").as_::<DOMString>()
    }

    pub fn set_alt(&mut self, value: DOMString) {
        self.inner.set("alt", value);
    }
}
impl HTMLAreaElement {
    pub fn coords(&self) -> DOMString {
        self.inner.get("coords").as_::<DOMString>()
    }

    pub fn set_coords(&mut self, value: DOMString) {
        self.inner.set("coords", value);
    }
}
impl HTMLAreaElement {
    pub fn shape(&self) -> DOMString {
        self.inner.get("shape").as_::<DOMString>()
    }

    pub fn set_shape(&mut self, value: DOMString) {
        self.inner.set("shape", value);
    }
}
impl HTMLAreaElement {
    pub fn target(&self) -> DOMString {
        self.inner.get("target").as_::<DOMString>()
    }

    pub fn set_target(&mut self, value: DOMString) {
        self.inner.set("target", value);
    }
}
impl HTMLAreaElement {
    pub fn download(&self) -> DOMString {
        self.inner.get("download").as_::<DOMString>()
    }

    pub fn set_download(&mut self, value: DOMString) {
        self.inner.set("download", value);
    }
}
impl HTMLAreaElement {
    pub fn ping(&self) -> USVString {
        self.inner.get("ping").as_::<USVString>()
    }

    pub fn set_ping(&mut self, value: USVString) {
        self.inner.set("ping", value);
    }
}
impl HTMLAreaElement {
    pub fn rel(&self) -> DOMString {
        self.inner.get("rel").as_::<DOMString>()
    }

    pub fn set_rel(&mut self, value: DOMString) {
        self.inner.set("rel", value);
    }
}
impl HTMLAreaElement {
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl HTMLAreaElement {
    pub fn referrer_policy(&self) -> DOMString {
        self.inner.get("referrerPolicy").as_::<DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLAreaElement {
    pub fn no_href(&self) -> bool {
        self.inner.get("noHref").as_::<bool>()
    }

    pub fn set_no_href(&mut self, value: bool) {
        self.inner.set("noHref", value);
    }
}
impl HTMLAreaElement {
    pub fn attribution_src(&self) -> USVString {
        self.inner.get("attributionSrc").as_::<USVString>()
    }

    pub fn set_attribution_src(&mut self, value: USVString) {
        self.inner.set("attributionSrc", value);
    }
}
impl HTMLAreaElement {
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }

    pub fn set_href(&mut self, value: USVString) {
        self.inner.set("href", value);
    }
}
impl HTMLAreaElement {
    pub fn origin(&self) -> USVString {
        self.inner.get("origin").as_::<USVString>()
    }
}
impl HTMLAreaElement {
    pub fn protocol(&self) -> USVString {
        self.inner.get("protocol").as_::<USVString>()
    }

    pub fn set_protocol(&mut self, value: USVString) {
        self.inner.set("protocol", value);
    }
}
impl HTMLAreaElement {
    pub fn username(&self) -> USVString {
        self.inner.get("username").as_::<USVString>()
    }

    pub fn set_username(&mut self, value: USVString) {
        self.inner.set("username", value);
    }
}
impl HTMLAreaElement {
    pub fn password(&self) -> USVString {
        self.inner.get("password").as_::<USVString>()
    }

    pub fn set_password(&mut self, value: USVString) {
        self.inner.set("password", value);
    }
}
impl HTMLAreaElement {
    pub fn host(&self) -> USVString {
        self.inner.get("host").as_::<USVString>()
    }

    pub fn set_host(&mut self, value: USVString) {
        self.inner.set("host", value);
    }
}
impl HTMLAreaElement {
    pub fn hostname(&self) -> USVString {
        self.inner.get("hostname").as_::<USVString>()
    }

    pub fn set_hostname(&mut self, value: USVString) {
        self.inner.set("hostname", value);
    }
}
impl HTMLAreaElement {
    pub fn port(&self) -> USVString {
        self.inner.get("port").as_::<USVString>()
    }

    pub fn set_port(&mut self, value: USVString) {
        self.inner.set("port", value);
    }
}
impl HTMLAreaElement {
    pub fn pathname(&self) -> USVString {
        self.inner.get("pathname").as_::<USVString>()
    }

    pub fn set_pathname(&mut self, value: USVString) {
        self.inner.set("pathname", value);
    }
}
impl HTMLAreaElement {
    pub fn search(&self) -> USVString {
        self.inner.get("search").as_::<USVString>()
    }

    pub fn set_search(&mut self, value: USVString) {
        self.inner.set("search", value);
    }
}
impl HTMLAreaElement {
    pub fn hash(&self) -> USVString {
        self.inner.get("hash").as_::<USVString>()
    }

    pub fn set_hash(&mut self, value: USVString) {
        self.inner.set("hash", value);
    }
}
