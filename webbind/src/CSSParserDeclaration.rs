use super::*;

/// The CSSParserDeclaration class.
/// [`CSSParserDeclaration`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserDeclaration)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSParserDeclaration {
    inner: CSSParserRule,
}
impl FromVal for CSSParserDeclaration {
    fn from_val(v: &Any) -> Self {
        CSSParserDeclaration {
            inner: CSSParserRule::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for CSSParserDeclaration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSParserDeclaration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSParserDeclaration> for Any {
    fn from(s: CSSParserDeclaration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSParserDeclaration> for Any {
    fn from(s: &CSSParserDeclaration) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSParserDeclaration);

impl CSSParserDeclaration {
    /// The `new CSSParserDeclaration(..)` constructor, creating a new CSSParserDeclaration instance
    pub fn new0(name: &JsString) -> CSSParserDeclaration {
        Self {
            inner: Any::global("CSSParserDeclaration")
                .new(&[name.into()])
                .as_::<CSSParserRule>(),
        }
    }

    /// The `new CSSParserDeclaration(..)` constructor, creating a new CSSParserDeclaration instance
    pub fn new1(name: &JsString, body: &TypedArray<CSSParserRule>) -> CSSParserDeclaration {
        Self {
            inner: Any::global("CSSParserDeclaration")
                .new(&[name.into(), body.into()])
                .as_::<CSSParserRule>(),
        }
    }
}
impl CSSParserDeclaration {
    /// Getter of the `name` attribute.
    /// [`CSSParserDeclaration.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserDeclaration/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl CSSParserDeclaration {
    /// Getter of the `body` attribute.
    /// [`CSSParserDeclaration.body`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserDeclaration/body)
    pub fn body(&self) -> TypedArray<CSSParserValue> {
        self.inner.get("body").as_::<TypedArray<CSSParserValue>>()
    }
}
