use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XMLSerializer {
    inner: emlite::Val,
}
impl FromVal for XMLSerializer {
    fn from_val(v: &emlite::Val) -> Self {
        XMLSerializer { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XMLSerializer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XMLSerializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XMLSerializer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XMLSerializer {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<XMLSerializer> for emlite::Val {
    fn from(s: XMLSerializer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XMLSerializer);



impl XMLSerializer {
    pub fn new() -> XMLSerializer {
        Self {
            inner: emlite::Val::global("XMLSerializer").new(&[]).as_::<emlite::Val>(),
        }
    }

}
impl XMLSerializer {
    pub fn serialize_to_string(&self, root: Node) -> DOMString {
        self.inner.call("serializeToString", &[root.into(), ]).as_::<DOMString>()
    }

}
