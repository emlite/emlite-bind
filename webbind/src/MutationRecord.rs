use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for MutationRecord {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MutationRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MutationRecord {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MutationRecord {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MutationRecord> for emlite::Val {
    fn from(s: MutationRecord) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MutationRecord);

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
