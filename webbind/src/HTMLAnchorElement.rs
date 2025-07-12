use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for HTMLAnchorElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLAnchorElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLAnchorElement> for emlite::Val {
    fn from(s: HTMLAnchorElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn target(&self) -> jsbind::DOMString {
        self.inner.get("target").as_::<jsbind::DOMString>()
    }

    pub fn set_target(&mut self, value: jsbind::DOMString) {
        self.inner.set("target", value);
    }
}
impl HTMLAnchorElement {
    pub fn download(&self) -> jsbind::DOMString {
        self.inner.get("download").as_::<jsbind::DOMString>()
    }

    pub fn set_download(&mut self, value: jsbind::DOMString) {
        self.inner.set("download", value);
    }
}
impl HTMLAnchorElement {
    pub fn ping(&self) -> jsbind::USVString {
        self.inner.get("ping").as_::<jsbind::USVString>()
    }

    pub fn set_ping(&mut self, value: jsbind::USVString) {
        self.inner.set("ping", value);
    }
}
impl HTMLAnchorElement {
    pub fn rel(&self) -> jsbind::DOMString {
        self.inner.get("rel").as_::<jsbind::DOMString>()
    }

    pub fn set_rel(&mut self, value: jsbind::DOMString) {
        self.inner.set("rel", value);
    }
}
impl HTMLAnchorElement {
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl HTMLAnchorElement {
    pub fn hreflang(&self) -> jsbind::DOMString {
        self.inner.get("hreflang").as_::<jsbind::DOMString>()
    }

    pub fn set_hreflang(&mut self, value: jsbind::DOMString) {
        self.inner.set("hreflang", value);
    }
}
impl HTMLAnchorElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLAnchorElement {
    pub fn text(&self) -> jsbind::DOMString {
        self.inner.get("text").as_::<jsbind::DOMString>()
    }

    pub fn set_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("text", value);
    }
}
impl HTMLAnchorElement {
    pub fn referrer_policy(&self) -> jsbind::DOMString {
        self.inner.get("referrerPolicy").as_::<jsbind::DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: jsbind::DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLAnchorElement {
    pub fn coords(&self) -> jsbind::DOMString {
        self.inner.get("coords").as_::<jsbind::DOMString>()
    }

    pub fn set_coords(&mut self, value: jsbind::DOMString) {
        self.inner.set("coords", value);
    }
}
impl HTMLAnchorElement {
    pub fn charset(&self) -> jsbind::DOMString {
        self.inner.get("charset").as_::<jsbind::DOMString>()
    }

    pub fn set_charset(&mut self, value: jsbind::DOMString) {
        self.inner.set("charset", value);
    }
}
impl HTMLAnchorElement {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
impl HTMLAnchorElement {
    pub fn rev(&self) -> jsbind::DOMString {
        self.inner.get("rev").as_::<jsbind::DOMString>()
    }

    pub fn set_rev(&mut self, value: jsbind::DOMString) {
        self.inner.set("rev", value);
    }
}
impl HTMLAnchorElement {
    pub fn shape(&self) -> jsbind::DOMString {
        self.inner.get("shape").as_::<jsbind::DOMString>()
    }

    pub fn set_shape(&mut self, value: jsbind::DOMString) {
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
    pub fn attribution_src(&self) -> jsbind::USVString {
        self.inner.get("attributionSrc").as_::<jsbind::USVString>()
    }

    pub fn set_attribution_src(&mut self, value: jsbind::USVString) {
        self.inner.set("attributionSrc", value);
    }
}
impl HTMLAnchorElement {
    pub fn href(&self) -> jsbind::USVString {
        self.inner.get("href").as_::<jsbind::USVString>()
    }

    pub fn set_href(&mut self, value: jsbind::USVString) {
        self.inner.set("href", value);
    }
}
impl HTMLAnchorElement {
    pub fn origin(&self) -> jsbind::USVString {
        self.inner.get("origin").as_::<jsbind::USVString>()
    }
}
impl HTMLAnchorElement {
    pub fn protocol(&self) -> jsbind::USVString {
        self.inner.get("protocol").as_::<jsbind::USVString>()
    }

    pub fn set_protocol(&mut self, value: jsbind::USVString) {
        self.inner.set("protocol", value);
    }
}
impl HTMLAnchorElement {
    pub fn username(&self) -> jsbind::USVString {
        self.inner.get("username").as_::<jsbind::USVString>()
    }

    pub fn set_username(&mut self, value: jsbind::USVString) {
        self.inner.set("username", value);
    }
}
impl HTMLAnchorElement {
    pub fn password(&self) -> jsbind::USVString {
        self.inner.get("password").as_::<jsbind::USVString>()
    }

    pub fn set_password(&mut self, value: jsbind::USVString) {
        self.inner.set("password", value);
    }
}
impl HTMLAnchorElement {
    pub fn host(&self) -> jsbind::USVString {
        self.inner.get("host").as_::<jsbind::USVString>()
    }

    pub fn set_host(&mut self, value: jsbind::USVString) {
        self.inner.set("host", value);
    }
}
impl HTMLAnchorElement {
    pub fn hostname(&self) -> jsbind::USVString {
        self.inner.get("hostname").as_::<jsbind::USVString>()
    }

    pub fn set_hostname(&mut self, value: jsbind::USVString) {
        self.inner.set("hostname", value);
    }
}
impl HTMLAnchorElement {
    pub fn port(&self) -> jsbind::USVString {
        self.inner.get("port").as_::<jsbind::USVString>()
    }

    pub fn set_port(&mut self, value: jsbind::USVString) {
        self.inner.set("port", value);
    }
}
impl HTMLAnchorElement {
    pub fn pathname(&self) -> jsbind::USVString {
        self.inner.get("pathname").as_::<jsbind::USVString>()
    }

    pub fn set_pathname(&mut self, value: jsbind::USVString) {
        self.inner.set("pathname", value);
    }
}
impl HTMLAnchorElement {
    pub fn search(&self) -> jsbind::USVString {
        self.inner.get("search").as_::<jsbind::USVString>()
    }

    pub fn set_search(&mut self, value: jsbind::USVString) {
        self.inner.set("search", value);
    }
}
impl HTMLAnchorElement {
    pub fn hash(&self) -> jsbind::USVString {
        self.inner.get("hash").as_::<jsbind::USVString>()
    }

    pub fn set_hash(&mut self, value: jsbind::USVString) {
        self.inner.set("hash", value);
    }
}
