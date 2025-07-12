use super::*;

#[derive(Clone, Debug)]
pub struct NamedNodeMap {
    inner: emlite::Val,
}
impl FromVal for NamedNodeMap {
    fn from_val(v: &emlite::Val) -> Self {
        NamedNodeMap {
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
impl std::ops::Deref for NamedNodeMap {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NamedNodeMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NamedNodeMap> for emlite::Val {
    fn from(s: NamedNodeMap) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NamedNodeMap {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl NamedNodeMap {
    pub fn item(&self, index: u32) -> Attr {
        self.inner.call("item", &[index.into()]).as_::<Attr>()
    }
}
impl NamedNodeMap {
    pub fn get_named_item(&self, qualified_name: jsbind::DOMString) -> Attr {
        self.inner
            .call("getNamedItem", &[qualified_name.into()])
            .as_::<Attr>()
    }
}
impl NamedNodeMap {
    pub fn get_named_item_ns(
        &self,
        namespace: jsbind::DOMString,
        local_name: jsbind::DOMString,
    ) -> Attr {
        self.inner
            .call("getNamedItemNS", &[namespace.into(), local_name.into()])
            .as_::<Attr>()
    }
}
impl NamedNodeMap {
    pub fn set_named_item(&self, attr: Attr) -> Attr {
        self.inner
            .call("setNamedItem", &[attr.into()])
            .as_::<Attr>()
    }
}
impl NamedNodeMap {
    pub fn set_named_item_ns(&self, attr: Attr) -> Attr {
        self.inner
            .call("setNamedItemNS", &[attr.into()])
            .as_::<Attr>()
    }
}
impl NamedNodeMap {
    pub fn remove_named_item(&self, qualified_name: jsbind::DOMString) -> Attr {
        self.inner
            .call("removeNamedItem", &[qualified_name.into()])
            .as_::<Attr>()
    }
}
impl NamedNodeMap {
    pub fn remove_named_item_ns(
        &self,
        namespace: jsbind::DOMString,
        local_name: jsbind::DOMString,
    ) -> Attr {
        self.inner
            .call("removeNamedItemNS", &[namespace.into(), local_name.into()])
            .as_::<Attr>()
    }
}
