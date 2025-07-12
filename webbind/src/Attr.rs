use super::*;

#[derive(Clone, Debug)]
pub struct Attr {
    inner: Node,
}
impl FromVal for Attr {
    fn from_val(v: &emlite::Val) -> Self {
        Attr {
            inner: Node::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for Attr {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Attr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Attr> for emlite::Val {
    fn from(s: Attr) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Attr {
    pub fn namespace_uri(&self) -> jsbind::DOMString {
        self.inner.get("namespaceURI").as_::<jsbind::DOMString>()
    }
}
impl Attr {
    pub fn prefix(&self) -> jsbind::DOMString {
        self.inner.get("prefix").as_::<jsbind::DOMString>()
    }
}
impl Attr {
    pub fn local_name(&self) -> jsbind::DOMString {
        self.inner.get("localName").as_::<jsbind::DOMString>()
    }
}
impl Attr {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl Attr {
    pub fn value(&self) -> jsbind::DOMString {
        self.inner.get("value").as_::<jsbind::DOMString>()
    }

    pub fn set_value(&mut self, value: jsbind::DOMString) {
        self.inner.set("value", value);
    }
}
impl Attr {
    pub fn owner_element(&self) -> Element {
        self.inner.get("ownerElement").as_::<Element>()
    }
}
impl Attr {
    pub fn specified(&self) -> bool {
        self.inner.get("specified").as_::<bool>()
    }
}
