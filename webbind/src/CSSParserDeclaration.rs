use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSParserDeclaration {
    inner: CSSParserRule,
}
impl FromVal for CSSParserDeclaration {
    fn from_val(v: &emlite::Val) -> Self {
        CSSParserDeclaration { inner: CSSParserRule::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSParserDeclaration {
    type Target = CSSParserRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSParserDeclaration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSParserDeclaration {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSParserDeclaration {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSParserDeclaration> for emlite::Val {
    fn from(s: CSSParserDeclaration) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSParserDeclaration);



impl CSSParserDeclaration {
    pub fn new0(name: DOMString) -> CSSParserDeclaration {
        Self {
            inner: emlite::Val::global("CSSParserDeclaration").new(&[name.into()]).as_::<CSSParserRule>(),
        }
    }

    pub fn new1(name: DOMString, body: Sequence<CSSParserRule>) -> CSSParserDeclaration {
        Self {
            inner: emlite::Val::global("CSSParserDeclaration").new(&[name.into(), body.into()]).as_::<CSSParserRule>(),
        }
    }

}
impl CSSParserDeclaration {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

}
impl CSSParserDeclaration {
    pub fn body(&self) -> FrozenArray<CSSParserValue> {
        self.inner.get("body").as_::<FrozenArray<CSSParserValue>>()
    }

}
