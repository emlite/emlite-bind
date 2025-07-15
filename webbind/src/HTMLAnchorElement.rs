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
impl From<&HTMLAnchorElement> for emlite::Val {
    fn from(s: &HTMLAnchorElement) -> emlite::Val {
        s.inner.clone().into()
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
    pub fn target(&self) -> String {
        self.inner.get("target").as_::<String>()
    }

    pub fn set_target(&mut self, value: &str) {
        self.inner.set("target", value);
    }
}
impl HTMLAnchorElement {
    pub fn download(&self) -> String {
        self.inner.get("download").as_::<String>()
    }

    pub fn set_download(&mut self, value: &str) {
        self.inner.set("download", value);
    }
}
impl HTMLAnchorElement {
    pub fn ping(&self) -> String {
        self.inner.get("ping").as_::<String>()
    }

    pub fn set_ping(&mut self, value: &str) {
        self.inner.set("ping", value);
    }
}
impl HTMLAnchorElement {
    pub fn rel(&self) -> String {
        self.inner.get("rel").as_::<String>()
    }

    pub fn set_rel(&mut self, value: &str) {
        self.inner.set("rel", value);
    }
}
impl HTMLAnchorElement {
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl HTMLAnchorElement {
    pub fn hreflang(&self) -> String {
        self.inner.get("hreflang").as_::<String>()
    }

    pub fn set_hreflang(&mut self, value: &str) {
        self.inner.set("hreflang", value);
    }
}
impl HTMLAnchorElement {
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }

    pub fn set_type_(&mut self, value: &str) {
        self.inner.set("type", value);
    }
}
impl HTMLAnchorElement {
    pub fn text(&self) -> String {
        self.inner.get("text").as_::<String>()
    }

    pub fn set_text(&mut self, value: &str) {
        self.inner.set("text", value);
    }
}
impl HTMLAnchorElement {
    pub fn referrer_policy(&self) -> String {
        self.inner.get("referrerPolicy").as_::<String>()
    }

    pub fn set_referrer_policy(&mut self, value: &str) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLAnchorElement {
    pub fn coords(&self) -> String {
        self.inner.get("coords").as_::<String>()
    }

    pub fn set_coords(&mut self, value: &str) {
        self.inner.set("coords", value);
    }
}
impl HTMLAnchorElement {
    pub fn charset(&self) -> String {
        self.inner.get("charset").as_::<String>()
    }

    pub fn set_charset(&mut self, value: &str) {
        self.inner.set("charset", value);
    }
}
impl HTMLAnchorElement {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl HTMLAnchorElement {
    pub fn rev(&self) -> String {
        self.inner.get("rev").as_::<String>()
    }

    pub fn set_rev(&mut self, value: &str) {
        self.inner.set("rev", value);
    }
}
impl HTMLAnchorElement {
    pub fn shape(&self) -> String {
        self.inner.get("shape").as_::<String>()
    }

    pub fn set_shape(&mut self, value: &str) {
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
    pub fn attribution_src(&self) -> String {
        self.inner.get("attributionSrc").as_::<String>()
    }

    pub fn set_attribution_src(&mut self, value: &str) {
        self.inner.set("attributionSrc", value);
    }
}
impl HTMLAnchorElement {
    pub fn href(&self) -> String {
        self.inner.get("href").as_::<String>()
    }

    pub fn set_href(&mut self, value: &str) {
        self.inner.set("href", value);
    }
}
impl HTMLAnchorElement {
    pub fn origin(&self) -> String {
        self.inner.get("origin").as_::<String>()
    }
}
impl HTMLAnchorElement {
    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }

    pub fn set_protocol(&mut self, value: &str) {
        self.inner.set("protocol", value);
    }
}
impl HTMLAnchorElement {
    pub fn username(&self) -> String {
        self.inner.get("username").as_::<String>()
    }

    pub fn set_username(&mut self, value: &str) {
        self.inner.set("username", value);
    }
}
impl HTMLAnchorElement {
    pub fn password(&self) -> String {
        self.inner.get("password").as_::<String>()
    }

    pub fn set_password(&mut self, value: &str) {
        self.inner.set("password", value);
    }
}
impl HTMLAnchorElement {
    pub fn host(&self) -> String {
        self.inner.get("host").as_::<String>()
    }

    pub fn set_host(&mut self, value: &str) {
        self.inner.set("host", value);
    }
}
impl HTMLAnchorElement {
    pub fn hostname(&self) -> String {
        self.inner.get("hostname").as_::<String>()
    }

    pub fn set_hostname(&mut self, value: &str) {
        self.inner.set("hostname", value);
    }
}
impl HTMLAnchorElement {
    pub fn port(&self) -> String {
        self.inner.get("port").as_::<String>()
    }

    pub fn set_port(&mut self, value: &str) {
        self.inner.set("port", value);
    }
}
impl HTMLAnchorElement {
    pub fn pathname(&self) -> String {
        self.inner.get("pathname").as_::<String>()
    }

    pub fn set_pathname(&mut self, value: &str) {
        self.inner.set("pathname", value);
    }
}
impl HTMLAnchorElement {
    pub fn search(&self) -> String {
        self.inner.get("search").as_::<String>()
    }

    pub fn set_search(&mut self, value: &str) {
        self.inner.set("search", value);
    }
}
impl HTMLAnchorElement {
    pub fn hash(&self) -> String {
        self.inner.get("hash").as_::<String>()
    }

    pub fn set_hash(&mut self, value: &str) {
        self.inner.set("hash", value);
    }
}
