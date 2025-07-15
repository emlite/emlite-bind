use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLAnchorElement {
    inner: HTMLElement,
}
impl FromVal for HTMLAnchorElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLAnchorElement {
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
impl core::ops::Deref for HTMLAnchorElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLAnchorElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLAnchorElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLAnchorElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLAnchorElement> for emlite::Val {
    fn from(s: HTMLAnchorElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLAnchorElement);

impl HTMLAnchorElement {
    pub fn new() -> HTMLAnchorElement {
        Self {
            inner: emlite::Val::global("HTMLAnchorElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLAnchorElement {
    pub fn target(&self) -> DOMString {
        self.inner.get("target").as_::<DOMString>()
    }

    pub fn set_target(&mut self, value: DOMString) {
        self.inner.set("target", value);
    }
}
impl HTMLAnchorElement {
    pub fn download(&self) -> DOMString {
        self.inner.get("download").as_::<DOMString>()
    }

    pub fn set_download(&mut self, value: DOMString) {
        self.inner.set("download", value);
    }
}
impl HTMLAnchorElement {
    pub fn ping(&self) -> USVString {
        self.inner.get("ping").as_::<USVString>()
    }

    pub fn set_ping(&mut self, value: USVString) {
        self.inner.set("ping", value);
    }
}
impl HTMLAnchorElement {
    pub fn rel(&self) -> DOMString {
        self.inner.get("rel").as_::<DOMString>()
    }

    pub fn set_rel(&mut self, value: DOMString) {
        self.inner.set("rel", value);
    }
}
impl HTMLAnchorElement {
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl HTMLAnchorElement {
    pub fn hreflang(&self) -> DOMString {
        self.inner.get("hreflang").as_::<DOMString>()
    }

    pub fn set_hreflang(&mut self, value: DOMString) {
        self.inner.set("hreflang", value);
    }
}
impl HTMLAnchorElement {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLAnchorElement {
    pub fn text(&self) -> DOMString {
        self.inner.get("text").as_::<DOMString>()
    }

    pub fn set_text(&mut self, value: DOMString) {
        self.inner.set("text", value);
    }
}
impl HTMLAnchorElement {
    pub fn referrer_policy(&self) -> DOMString {
        self.inner.get("referrerPolicy").as_::<DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLAnchorElement {
    pub fn coords(&self) -> DOMString {
        self.inner.get("coords").as_::<DOMString>()
    }

    pub fn set_coords(&mut self, value: DOMString) {
        self.inner.set("coords", value);
    }
}
impl HTMLAnchorElement {
    pub fn charset(&self) -> DOMString {
        self.inner.get("charset").as_::<DOMString>()
    }

    pub fn set_charset(&mut self, value: DOMString) {
        self.inner.set("charset", value);
    }
}
impl HTMLAnchorElement {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

    pub fn set_name(&mut self, value: DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLAnchorElement {
    pub fn rev(&self) -> DOMString {
        self.inner.get("rev").as_::<DOMString>()
    }

    pub fn set_rev(&mut self, value: DOMString) {
        self.inner.set("rev", value);
    }
}
impl HTMLAnchorElement {
    pub fn shape(&self) -> DOMString {
        self.inner.get("shape").as_::<DOMString>()
    }

    pub fn set_shape(&mut self, value: DOMString) {
        self.inner.set("shape", value);
    }
}
impl HTMLAnchorElement {
    pub fn attribution_source_id(&self) -> u32 {
        self.inner.get("attributionSourceId").as_::<u32>()
    }

    pub fn set_attribution_source_id(&mut self, value: u32) {
        self.inner.set("attributionSourceId", value);
    }
}
impl HTMLAnchorElement {
    pub fn attribution_src(&self) -> USVString {
        self.inner.get("attributionSrc").as_::<USVString>()
    }

    pub fn set_attribution_src(&mut self, value: USVString) {
        self.inner.set("attributionSrc", value);
    }
}
impl HTMLAnchorElement {
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }

    pub fn set_href(&mut self, value: USVString) {
        self.inner.set("href", value);
    }
}
impl HTMLAnchorElement {
    pub fn origin(&self) -> USVString {
        self.inner.get("origin").as_::<USVString>()
    }
}
impl HTMLAnchorElement {
    pub fn protocol(&self) -> USVString {
        self.inner.get("protocol").as_::<USVString>()
    }

    pub fn set_protocol(&mut self, value: USVString) {
        self.inner.set("protocol", value);
    }
}
impl HTMLAnchorElement {
    pub fn username(&self) -> USVString {
        self.inner.get("username").as_::<USVString>()
    }

    pub fn set_username(&mut self, value: USVString) {
        self.inner.set("username", value);
    }
}
impl HTMLAnchorElement {
    pub fn password(&self) -> USVString {
        self.inner.get("password").as_::<USVString>()
    }

    pub fn set_password(&mut self, value: USVString) {
        self.inner.set("password", value);
    }
}
impl HTMLAnchorElement {
    pub fn host(&self) -> USVString {
        self.inner.get("host").as_::<USVString>()
    }

    pub fn set_host(&mut self, value: USVString) {
        self.inner.set("host", value);
    }
}
impl HTMLAnchorElement {
    pub fn hostname(&self) -> USVString {
        self.inner.get("hostname").as_::<USVString>()
    }

    pub fn set_hostname(&mut self, value: USVString) {
        self.inner.set("hostname", value);
    }
}
impl HTMLAnchorElement {
    pub fn port(&self) -> USVString {
        self.inner.get("port").as_::<USVString>()
    }

    pub fn set_port(&mut self, value: USVString) {
        self.inner.set("port", value);
    }
}
impl HTMLAnchorElement {
    pub fn pathname(&self) -> USVString {
        self.inner.get("pathname").as_::<USVString>()
    }

    pub fn set_pathname(&mut self, value: USVString) {
        self.inner.set("pathname", value);
    }
}
impl HTMLAnchorElement {
    pub fn search(&self) -> USVString {
        self.inner.get("search").as_::<USVString>()
    }

    pub fn set_search(&mut self, value: USVString) {
        self.inner.set("search", value);
    }
}
impl HTMLAnchorElement {
    pub fn hash(&self) -> USVString {
        self.inner.get("hash").as_::<USVString>()
    }

    pub fn set_hash(&mut self, value: USVString) {
        self.inner.set("hash", value);
    }
}
