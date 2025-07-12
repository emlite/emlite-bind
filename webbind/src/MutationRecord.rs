use super::*;

#[derive(Clone, Debug)]
pub struct MutationRecord {
    inner: emlite::Val,
}
impl FromVal for MutationRecord {
    fn from_val(v: &emlite::Val) -> Self {
        MutationRecord {
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
impl std::ops::Deref for MutationRecord {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MutationRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MutationRecord> for emlite::Val {
    fn from(s: MutationRecord) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MutationRecord {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }
}
impl MutationRecord {
    pub fn target(&self) -> Node {
        self.inner.get("target").as_::<Node>()
    }
}
impl MutationRecord {
    pub fn added_nodes(&self) -> NodeList {
        self.inner.get("addedNodes").as_::<NodeList>()
    }
}
impl MutationRecord {
    pub fn removed_nodes(&self) -> NodeList {
        self.inner.get("removedNodes").as_::<NodeList>()
    }
}
impl MutationRecord {
    pub fn previous_sibling(&self) -> Node {
        self.inner.get("previousSibling").as_::<Node>()
    }
}
impl MutationRecord {
    pub fn next_sibling(&self) -> Node {
        self.inner.get("nextSibling").as_::<Node>()
    }
}
impl MutationRecord {
    pub fn attribute_name(&self) -> jsbind::DOMString {
        self.inner.get("attributeName").as_::<jsbind::DOMString>()
    }
}
impl MutationRecord {
    pub fn attribute_namespace(&self) -> jsbind::DOMString {
        self.inner
            .get("attributeNamespace")
            .as_::<jsbind::DOMString>()
    }
}
impl MutationRecord {
    pub fn old_value(&self) -> jsbind::DOMString {
        self.inner.get("oldValue").as_::<jsbind::DOMString>()
    }
}
