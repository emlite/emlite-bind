use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentType {
    inner: Node,
}
impl FromVal for DocumentType {
    fn from_val(v: &emlite::Val) -> Self {
        DocumentType {
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
impl core::ops::Deref for DocumentType {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DocumentType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DocumentType {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DocumentType {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DocumentType> for emlite::Val {
    fn from(s: DocumentType) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&DocumentType> for emlite::Val {
    fn from(s: &DocumentType) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DocumentType);

impl DocumentType {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl DocumentType {
    pub fn public_id(&self) -> DOMString {
        self.inner.get("publicId").as_::<DOMString>()
    }
}
impl DocumentType {
    pub fn system_id(&self) -> DOMString {
        self.inner.get("systemId").as_::<DOMString>()
    }
}
impl DocumentType {
    pub fn before(&self, nodes: Any) -> Undefined {
        self.inner
            .call("before", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl DocumentType {
    pub fn after(&self, nodes: Any) -> Undefined {
        self.inner.call("after", &[nodes.into()]).as_::<Undefined>()
    }
}
impl DocumentType {
    pub fn replace_with(&self, nodes: Any) -> Undefined {
        self.inner
            .call("replaceWith", &[nodes.into()])
            .as_::<Undefined>()
    }
}
impl DocumentType {
    pub fn remove(&self) -> Undefined {
        self.inner.call("remove", &[]).as_::<Undefined>()
    }
}
