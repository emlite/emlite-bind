use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NamedNodeMap {
    inner: emlite::Val,
}
impl FromVal for NamedNodeMap {
    fn from_val(v: &emlite::Val) -> Self {
        NamedNodeMap { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NamedNodeMap {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NamedNodeMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NamedNodeMap {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NamedNodeMap {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<NamedNodeMap> for emlite::Val {
    fn from(s: NamedNodeMap) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NamedNodeMap);


impl NamedNodeMap {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl NamedNodeMap {
    pub fn item(&self, index: u32) -> Attr {
        self.inner.call("item", &[index.into(), ]).as_::<Attr>()
    }

}
impl NamedNodeMap {
    pub fn get_named_item(&self, qualified_name: DOMString) -> Attr {
        self.inner.call("getNamedItem", &[qualified_name.into(), ]).as_::<Attr>()
    }

}
impl NamedNodeMap {
    pub fn get_named_item_ns(&self, namespace: DOMString, local_name: DOMString) -> Attr {
        self.inner.call("getNamedItemNS", &[namespace.into(), local_name.into(), ]).as_::<Attr>()
    }

}
impl NamedNodeMap {
    pub fn set_named_item(&self, attr: Attr) -> Attr {
        self.inner.call("setNamedItem", &[attr.into(), ]).as_::<Attr>()
    }

}
impl NamedNodeMap {
    pub fn set_named_item_ns(&self, attr: Attr) -> Attr {
        self.inner.call("setNamedItemNS", &[attr.into(), ]).as_::<Attr>()
    }

}
impl NamedNodeMap {
    pub fn remove_named_item(&self, qualified_name: DOMString) -> Attr {
        self.inner.call("removeNamedItem", &[qualified_name.into(), ]).as_::<Attr>()
    }

}
impl NamedNodeMap {
    pub fn remove_named_item_ns(&self, namespace: DOMString, local_name: DOMString) -> Attr {
        self.inner.call("removeNamedItemNS", &[namespace.into(), local_name.into(), ]).as_::<Attr>()
    }

}
