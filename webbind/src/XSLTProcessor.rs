use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XSLTProcessor {
    inner: emlite::Val,
}
impl FromVal for XSLTProcessor {
    fn from_val(v: &emlite::Val) -> Self {
        XSLTProcessor { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XSLTProcessor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XSLTProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XSLTProcessor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XSLTProcessor {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<XSLTProcessor> for emlite::Val {
    fn from(s: XSLTProcessor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XSLTProcessor);



impl XSLTProcessor {
    pub fn new() -> XSLTProcessor {
        Self {
            inner: emlite::Val::global("XSLTProcessor").new(&[]).as_::<emlite::Val>(),
        }
    }

}
impl XSLTProcessor {
    pub fn import_stylesheet(&self, style: Node) -> Undefined {
        self.inner.call("importStylesheet", &[style.into(), ]).as_::<Undefined>()
    }

}
impl XSLTProcessor {
    pub fn transform_to_fragment(&self, source: Node, output: Document) -> DocumentFragment {
        self.inner.call("transformToFragment", &[source.into(), output.into(), ]).as_::<DocumentFragment>()
    }

}
impl XSLTProcessor {
    pub fn transform_to_document(&self, source: Node) -> Document {
        self.inner.call("transformToDocument", &[source.into(), ]).as_::<Document>()
    }

}
impl XSLTProcessor {
    pub fn set_parameter(&self, namespace_uri: DOMString, local_name: DOMString, value: Any) -> Undefined {
        self.inner.call("setParameter", &[namespace_uri.into(), local_name.into(), value.into(), ]).as_::<Undefined>()
    }

}
impl XSLTProcessor {
    pub fn get_parameter(&self, namespace_uri: DOMString, local_name: DOMString) -> Any {
        self.inner.call("getParameter", &[namespace_uri.into(), local_name.into(), ]).as_::<Any>()
    }

}
impl XSLTProcessor {
    pub fn remove_parameter(&self, namespace_uri: DOMString, local_name: DOMString) -> Undefined {
        self.inner.call("removeParameter", &[namespace_uri.into(), local_name.into(), ]).as_::<Undefined>()
    }

}
impl XSLTProcessor {
    pub fn clear_parameters(&self, ) -> Undefined {
        self.inner.call("clearParameters", &[]).as_::<Undefined>()
    }

}
impl XSLTProcessor {
    pub fn reset(&self, ) -> Undefined {
        self.inner.call("reset", &[]).as_::<Undefined>()
    }

}
