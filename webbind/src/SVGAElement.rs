use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for SVGAElement {
    type Target = SVGGraphicsElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGAElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGAElement> for emlite::Val {
    fn from(s: SVGAElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGAElement {
    pub fn target(&self) -> SVGAnimatedString {
        self.inner.get("target").as_::<SVGAnimatedString>()
    }
}
impl SVGAElement {
    pub fn download(&self) -> jsbind::DOMString {
        self.inner.get("download").as_::<jsbind::DOMString>()
    }

    pub fn set_download(&mut self, value: jsbind::DOMString) {
        self.inner.set("download", value);
    }
}
impl SVGAElement {
    pub fn ping(&self) -> jsbind::USVString {
        self.inner.get("ping").as_::<jsbind::USVString>()
    }

    pub fn set_ping(&mut self, value: jsbind::USVString) {
        self.inner.set("ping", value);
    }
}
impl SVGAElement {
    pub fn rel(&self) -> jsbind::DOMString {
        self.inner.get("rel").as_::<jsbind::DOMString>()
    }

    pub fn set_rel(&mut self, value: jsbind::DOMString) {
        self.inner.set("rel", value);
    }
}
impl SVGAElement {
    pub fn rel_list(&self) -> DOMTokenList {
        self.inner.get("relList").as_::<DOMTokenList>()
    }
}
impl SVGAElement {
    pub fn hreflang(&self) -> jsbind::DOMString {
        self.inner.get("hreflang").as_::<jsbind::DOMString>()
    }

    pub fn set_hreflang(&mut self, value: jsbind::DOMString) {
        self.inner.set("hreflang", value);
    }
}
impl SVGAElement {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
        self.inner.set("type", value);
    }
}
impl SVGAElement {
    pub fn text(&self) -> jsbind::DOMString {
        self.inner.get("text").as_::<jsbind::DOMString>()
    }

    pub fn set_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("text", value);
    }
}
impl SVGAElement {
    pub fn referrer_policy(&self) -> jsbind::DOMString {
        self.inner.get("referrerPolicy").as_::<jsbind::DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: jsbind::DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl SVGAElement {
    pub fn origin(&self) -> jsbind::USVString {
        self.inner.get("origin").as_::<jsbind::USVString>()
    }
}
impl SVGAElement {
    pub fn protocol(&self) -> jsbind::USVString {
        self.inner.get("protocol").as_::<jsbind::USVString>()
    }

    pub fn set_protocol(&mut self, value: jsbind::USVString) {
        self.inner.set("protocol", value);
    }
}
impl SVGAElement {
    pub fn username(&self) -> jsbind::USVString {
        self.inner.get("username").as_::<jsbind::USVString>()
    }

    pub fn set_username(&mut self, value: jsbind::USVString) {
        self.inner.set("username", value);
    }
}
impl SVGAElement {
    pub fn password(&self) -> jsbind::USVString {
        self.inner.get("password").as_::<jsbind::USVString>()
    }

    pub fn set_password(&mut self, value: jsbind::USVString) {
        self.inner.set("password", value);
    }
}
impl SVGAElement {
    pub fn host(&self) -> jsbind::USVString {
        self.inner.get("host").as_::<jsbind::USVString>()
    }

    pub fn set_host(&mut self, value: jsbind::USVString) {
        self.inner.set("host", value);
    }
}
impl SVGAElement {
    pub fn hostname(&self) -> jsbind::USVString {
        self.inner.get("hostname").as_::<jsbind::USVString>()
    }

    pub fn set_hostname(&mut self, value: jsbind::USVString) {
        self.inner.set("hostname", value);
    }
}
impl SVGAElement {
    pub fn port(&self) -> jsbind::USVString {
        self.inner.get("port").as_::<jsbind::USVString>()
    }

    pub fn set_port(&mut self, value: jsbind::USVString) {
        self.inner.set("port", value);
    }
}
impl SVGAElement {
    pub fn pathname(&self) -> jsbind::USVString {
        self.inner.get("pathname").as_::<jsbind::USVString>()
    }

    pub fn set_pathname(&mut self, value: jsbind::USVString) {
        self.inner.set("pathname", value);
    }
}
impl SVGAElement {
    pub fn search(&self) -> jsbind::USVString {
        self.inner.get("search").as_::<jsbind::USVString>()
    }

    pub fn set_search(&mut self, value: jsbind::USVString) {
        self.inner.set("search", value);
    }
}
impl SVGAElement {
    pub fn hash(&self) -> jsbind::USVString {
        self.inner.get("hash").as_::<jsbind::USVString>()
    }

    pub fn set_hash(&mut self, value: jsbind::USVString) {
        self.inner.set("hash", value);
    }
}
impl SVGAElement {
    pub fn href(&self) -> SVGAnimatedString {
        self.inner.get("href").as_::<SVGAnimatedString>()
    }
}
