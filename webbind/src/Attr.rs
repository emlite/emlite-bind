use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Attr {
    inner: Node,
}
impl FromVal for Attr {
    fn from_val(v: &emlite::Val) -> Self {
        Attr { inner: Node::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Attr {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Attr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Attr {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Attr {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<Attr> for emlite::Val {
    fn from(s: Attr) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Attr);


impl Attr {
    pub fn namespace_uri(&self) -> DOMString {
        self.inner.get("namespaceURI").as_::<DOMString>()
    }

}
impl Attr {
    pub fn prefix(&self) -> DOMString {
        self.inner.get("prefix").as_::<DOMString>()
    }

}
impl Attr {
    pub fn local_name(&self) -> DOMString {
        self.inner.get("localName").as_::<DOMString>()
    }

}
impl Attr {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

}
impl Attr {
    pub fn value(&self) -> DOMString {
        self.inner.get("value").as_::<DOMString>()
    }

    pub fn set_value(&mut self, value: DOMString) {
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
