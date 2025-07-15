use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSParserBlock {
    inner: CSSParserValue,
}
impl FromVal for CSSParserBlock {
    fn from_val(v: &emlite::Val) -> Self {
        CSSParserBlock { inner: CSSParserValue::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSParserBlock {
    type Target = CSSParserValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSParserBlock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSParserBlock {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSParserBlock {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSParserBlock> for emlite::Val {
    fn from(s: CSSParserBlock) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSParserBlock);



impl CSSParserBlock {
    pub fn new(name: DOMString, body: Sequence<CSSParserValue>) -> CSSParserBlock {
        Self {
            inner: emlite::Val::global("CSSParserBlock").new(&[name.into(), body.into()]).as_::<CSSParserValue>(),
        }
    }

}
impl CSSParserBlock {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

}
impl CSSParserBlock {
    pub fn body(&self) -> FrozenArray<CSSParserValue> {
        self.inner.get("body").as_::<FrozenArray<CSSParserValue>>()
    }

}
