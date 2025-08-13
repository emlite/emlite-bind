use super::*;




/// The DocumentType class.
/// [`DocumentType`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentType {
    inner: Node,
}

impl FromVal for DocumentType {
    fn from_val(v: &Any) -> Self {
        DocumentType { inner: Node::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for DocumentType {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DocumentType {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DocumentType> for Any {
    fn from(s: DocumentType) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DocumentType> for Any {
    fn from(s: &DocumentType) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DocumentType);


impl DocumentType {
    /// Getter of the `name` attribute.
    /// [`DocumentType.name`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

}
impl DocumentType {
    /// Getter of the `publicId` attribute.
    /// [`DocumentType.publicId`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/publicId)
    pub fn public_id(&self) -> JsString {
        self.inner.get("publicId").as_::<JsString>()
    }

}
impl DocumentType {
    /// Getter of the `systemId` attribute.
    /// [`DocumentType.systemId`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/systemId)
    pub fn system_id(&self) -> JsString {
        self.inner.get("systemId").as_::<JsString>()
    }

}
impl DocumentType {
    /// The before method.
    /// [`DocumentType.before`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/before)
    pub fn before(&self, nodes: &Any) -> Undefined {
        self.inner.call("before", &[nodes.into(), ]).as_::<Undefined>()
    }
}
impl DocumentType {
    /// The after method.
    /// [`DocumentType.after`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/after)
    pub fn after(&self, nodes: &Any) -> Undefined {
        self.inner.call("after", &[nodes.into(), ]).as_::<Undefined>()
    }
}
impl DocumentType {
    /// The replaceWith method.
    /// [`DocumentType.replaceWith`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/replaceWith)
    pub fn replace_with(&self, nodes: &Any) -> Undefined {
        self.inner.call("replaceWith", &[nodes.into(), ]).as_::<Undefined>()
    }
}
impl DocumentType {
    /// The remove method.
    /// [`DocumentType.remove`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentType/remove)
    pub fn remove(&self, ) -> Undefined {
        self.inner.call("remove", &[]).as_::<Undefined>()
    }
}
