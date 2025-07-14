use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMImplementation {
    inner: emlite::Val,
}
impl FromVal for DOMImplementation {
    fn from_val(v: &emlite::Val) -> Self {
        DOMImplementation {
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
impl core::ops::Deref for DOMImplementation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMImplementation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMImplementation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMImplementation {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMImplementation> for emlite::Val {
    fn from(s: DOMImplementation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DOMImplementation);

impl DOMImplementation {
    pub fn create_document_type(
        &self,
        name: jsbind::DOMString,
        public_id: jsbind::DOMString,
        system_id: jsbind::DOMString,
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
    pub fn create_document0(
        &self,
        namespace: jsbind::DOMString,
        qualified_name: jsbind::DOMString,
    ) -> XMLDocument {
        self.inner
            .call("createDocument", &[namespace.into(), qualified_name.into()])
            .as_::<XMLDocument>()
    }

    pub fn create_document1(
        &self,
        namespace: jsbind::DOMString,
        qualified_name: jsbind::DOMString,
        doctype: DocumentType,
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
    pub fn create_html_document0(&self) -> Document {
        self.inner.call("createHTMLDocument", &[]).as_::<Document>()
    }

    pub fn create_html_document1(&self, title: jsbind::DOMString) -> Document {
        self.inner
            .call("createHTMLDocument", &[title.into()])
            .as_::<Document>()
    }
}
impl DOMImplementation {
    pub fn has_feature(&self) -> bool {
        self.inner.call("hasFeature", &[]).as_::<bool>()
    }
}
