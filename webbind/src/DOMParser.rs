use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMParser {
    inner: emlite::Val,
}
impl FromVal for DOMParser {
    fn from_val(v: &emlite::Val) -> Self {
        DOMParser { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DOMParser {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMParser {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMParser {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<DOMParser> for emlite::Val {
    fn from(s: DOMParser) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DOMParser);



impl DOMParser {
    pub fn new() -> DOMParser {
        Self {
            inner: emlite::Val::global("DOMParser").new(&[]).as_::<emlite::Val>(),
        }
    }

}
impl DOMParser {
    pub fn parse_from_string(&self, string: Any, type_: DOMParserSupportedType) -> Document {
        self.inner.call("parseFromString", &[string.into(), type_.into(), ]).as_::<Document>()
    }

}
