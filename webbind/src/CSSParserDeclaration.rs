use super::*;

#[derive(Clone, Debug)]
pub struct CSSParserDeclaration {
    inner: CSSParserRule,
}
impl FromVal for CSSParserDeclaration {
    fn from_val(v: &emlite::Val) -> Self {
        CSSParserDeclaration {
            inner: CSSParserRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSParserDeclaration {
    type Target = CSSParserRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSParserDeclaration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSParserDeclaration> for emlite::Val {
    fn from(s: CSSParserDeclaration) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSParserDeclaration {
    pub fn new0(name: jsbind::DOMString) -> CSSParserDeclaration {
        Self {
            inner: emlite::Val::global("CSSParserDeclaration")
                .new(&[name.into()])
                .as_::<CSSParserRule>(),
        }
    }

    pub fn new1(
        name: jsbind::DOMString,
        body: jsbind::Sequence<CSSParserRule>,
    ) -> CSSParserDeclaration {
        Self {
            inner: emlite::Val::global("CSSParserDeclaration")
                .new(&[name.into(), body.into()])
                .as_::<CSSParserRule>(),
        }
    }
}
impl CSSParserDeclaration {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl CSSParserDeclaration {
    pub fn body(&self) -> jsbind::FrozenArray<CSSParserValue> {
        self.inner
            .get("body")
            .as_::<jsbind::FrozenArray<CSSParserValue>>()
    }
}
