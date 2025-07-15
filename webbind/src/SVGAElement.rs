use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAElement {
    inner: SVGGraphicsElement,
}
impl FromVal for SVGAElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAElement {
            inner: SVGGraphicsElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGAElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGAElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGAElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGAElement> for emlite::Val {
    fn from(s: SVGAElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SVGAElement> for emlite::Val {
    fn from(s: &SVGAElement) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAElement);

impl SVGAElement {
    pub fn target(&self) -> SVGAnimatedString {
        self.inner.get("target").as_::<SVGAnimatedString>()
    }
}
impl SVGAElement {
    pub fn download(&self) -> String {
        self.inner.get("download").as_::<String>()
    }

    pub fn set_download(&mut self, value: &str) {
        self.inner.set("download", value);
    }
}
impl SVGAElement {
    pub fn ping(&self) -> String {
        self.inner.get("ping").as_::<String>()
    }

    pub fn set_ping(&mut self, value: &str) {
        self.inner.set("ping", value);
    }
}
impl SVGAElement {
    pub fn rel(&self) -> String {
        self.inner.get("rel").as_::<String>()
    }

    pub fn set_rel(&mut self, value: &str) {
        self.inner.set("rel", value);
    }
}
impl SVGAElement {
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl SVGAElement {
    pub fn hreflang(&self) -> String {
        self.inner.get("hreflang").as_::<String>()
    }

    pub fn set_hreflang(&mut self, value: &str) {
        self.inner.set("hreflang", value);
    }
}
impl SVGAElement {
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }

    pub fn set_type_(&mut self, value: &str) {
        self.inner.set("type", value);
    }
}
impl SVGAElement {
    pub fn text(&self) -> String {
        self.inner.get("text").as_::<String>()
    }

    pub fn set_text(&mut self, value: &str) {
        self.inner.set("text", value);
    }
}
impl SVGAElement {
    pub fn referrer_policy(&self) -> String {
        self.inner.get("referrerPolicy").as_::<String>()
    }

    pub fn set_referrer_policy(&mut self, value: &str) {
        self.inner.set("referrerPolicy", value);
    }
}
impl SVGAElement {
    pub fn origin(&self) -> String {
        self.inner.get("origin").as_::<String>()
    }
}
impl SVGAElement {
    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }

    pub fn set_protocol(&mut self, value: &str) {
        self.inner.set("protocol", value);
    }
}
impl SVGAElement {
    pub fn username(&self) -> String {
        self.inner.get("username").as_::<String>()
    }

    pub fn set_username(&mut self, value: &str) {
        self.inner.set("username", value);
    }
}
impl SVGAElement {
    pub fn password(&self) -> String {
        self.inner.get("password").as_::<String>()
    }

    pub fn set_password(&mut self, value: &str) {
        self.inner.set("password", value);
    }
}
impl SVGAElement {
    pub fn host(&self) -> String {
        self.inner.get("host").as_::<String>()
    }

    pub fn set_host(&mut self, value: &str) {
        self.inner.set("host", value);
    }
}
impl SVGAElement {
    pub fn hostname(&self) -> String {
        self.inner.get("hostname").as_::<String>()
    }

    pub fn set_hostname(&mut self, value: &str) {
        self.inner.set("hostname", value);
    }
}
impl SVGAElement {
    pub fn port(&self) -> String {
        self.inner.get("port").as_::<String>()
    }

    pub fn set_port(&mut self, value: &str) {
        self.inner.set("port", value);
    }
}
impl SVGAElement {
    pub fn pathname(&self) -> String {
        self.inner.get("pathname").as_::<String>()
    }

    pub fn set_pathname(&mut self, value: &str) {
        self.inner.set("pathname", value);
    }
}
impl SVGAElement {
    pub fn search(&self) -> String {
        self.inner.get("search").as_::<String>()
    }

    pub fn set_search(&mut self, value: &str) {
        self.inner.set("search", value);
    }
}
impl SVGAElement {
    pub fn hash(&self) -> String {
        self.inner.get("hash").as_::<String>()
    }

    pub fn set_hash(&mut self, value: &str) {
        self.inner.set("hash", value);
    }
}
impl SVGAElement {
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
