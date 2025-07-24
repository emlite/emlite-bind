use super::*;

/// The DOMImplementation class.
/// [`DOMImplementation`](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMImplementation {
    inner: Any,
}
impl FromVal for DOMImplementation {
    fn from_val(v: &Any) -> Self {
        DOMImplementation {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DOMImplementation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMImplementation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DOMImplementation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DOMImplementation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DOMImplementation> for Any {
    fn from(s: DOMImplementation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DOMImplementation> for Any {
    fn from(s: &DOMImplementation) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DOMImplementation);

impl DOMImplementation {
    /// The createDocumentType method.
    /// [`DOMImplementation.createDocumentType`](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocumentType)
    pub fn create_document_type(
        &self,
        name: &DOMString,
        public_id: &DOMString,
        system_id: &DOMString,
    ) -> DocumentType {
        self.inner
            .call(
                "createDocumentType",
                &[name.into(), public_id.into(), system_id.into()],
            )
            .as_::<DocumentType>()
    }
}
impl DOMImplementation {
    /// The createDocument method.
    /// [`DOMImplementation.createDocument`](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocument)
    pub fn create_document0(
        &self,
        namespace: &DOMString,
        qualified_name: &DOMString,
    ) -> XMLDocument {
        self.inner
            .call("createDocument", &[namespace.into(), qualified_name.into()])
            .as_::<XMLDocument>()
    }
    /// The createDocument method.
    /// [`DOMImplementation.createDocument`](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocument)
    pub fn create_document1(
        &self,
        namespace: &DOMString,
        qualified_name: &DOMString,
        doctype: &DocumentType,
    ) -> XMLDocument {
        self.inner
            .call(
                "createDocument",
                &[namespace.into(), qualified_name.into(), doctype.into()],
            )
            .as_::<XMLDocument>()
    }
}
impl DOMImplementation {
    /// The createHTMLDocument method.
    /// [`DOMImplementation.createHTMLDocument`](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createHTMLDocument)
    pub fn create_html_document0(&self) -> Document {
        self.inner.call("createHTMLDocument", &[]).as_::<Document>()
    }
    /// The createHTMLDocument method.
    /// [`DOMImplementation.createHTMLDocument`](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createHTMLDocument)
    pub fn create_html_document1(&self, title: &DOMString) -> Document {
        self.inner
            .call("createHTMLDocument", &[title.into()])
            .as_::<Document>()
    }
}
impl DOMImplementation {
    /// The hasFeature method.
    /// [`DOMImplementation.hasFeature`](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/hasFeature)
    pub fn has_feature(&self) -> bool {
        self.inner.call("hasFeature", &[]).as_::<bool>()
    }
}
