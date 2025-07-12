use super::*;

#[derive(Clone, Debug)]
pub struct XSLTProcessor {
    inner: emlite::Val,
}
impl FromVal for XSLTProcessor {
    fn from_val(v: &emlite::Val) -> Self {
        XSLTProcessor {
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
impl std::ops::Deref for XSLTProcessor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XSLTProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XSLTProcessor> for emlite::Val {
    fn from(s: XSLTProcessor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XSLTProcessor {
    pub fn new() -> XSLTProcessor {
        Self {
            inner: emlite::Val::global("XSLTProcessor")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl XSLTProcessor {
    pub fn import_stylesheet(&self, style: Node) -> jsbind::Undefined {
        self.inner
            .call("importStylesheet", &[style.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl XSLTProcessor {
    pub fn transform_to_fragment(&self, source: Node, output: Document) -> DocumentFragment {
        self.inner
            .call("transformToFragment", &[source.into(), output.into()])
            .as_::<DocumentFragment>()
    }
}
impl XSLTProcessor {
    pub fn transform_to_document(&self, source: Node) -> Document {
        self.inner
            .call("transformToDocument", &[source.into()])
            .as_::<Document>()
    }
}
impl XSLTProcessor {
    pub fn set_parameter(
        &self,
        namespace_uri: jsbind::DOMString,
        local_name: jsbind::DOMString,
        value: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "setParameter",
                &[namespace_uri.into(), local_name.into(), value.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl XSLTProcessor {
    pub fn get_parameter(
        &self,
        namespace_uri: jsbind::DOMString,
        local_name: jsbind::DOMString,
    ) -> jsbind::Any {
        self.inner
            .call("getParameter", &[namespace_uri.into(), local_name.into()])
            .as_::<jsbind::Any>()
    }
}
impl XSLTProcessor {
    pub fn remove_parameter(
        &self,
        namespace_uri: jsbind::DOMString,
        local_name: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "removeParameter",
                &[namespace_uri.into(), local_name.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl XSLTProcessor {
    pub fn clear_parameters(&self) -> jsbind::Undefined {
        self.inner
            .call("clearParameters", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl XSLTProcessor {
    pub fn reset(&self) -> jsbind::Undefined {
        self.inner.call("reset", &[]).as_::<jsbind::Undefined>()
    }
}
