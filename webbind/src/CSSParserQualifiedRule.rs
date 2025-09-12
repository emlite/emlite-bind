use super::*;

/// The CSSParserQualifiedRule class.
/// [`CSSParserQualifiedRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserQualifiedRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSParserQualifiedRule {
    inner: CSSParserRule,
}

impl FromVal for CSSParserQualifiedRule {
    fn from_val(v: &Any) -> Self {
        CSSParserQualifiedRule {
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

impl core::ops::Deref for CSSParserQualifiedRule {
    type Target = CSSParserRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSParserQualifiedRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSParserQualifiedRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSParserQualifiedRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSParserQualifiedRule> for Any {
    fn from(s: CSSParserQualifiedRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSParserQualifiedRule> for Any {
    fn from(s: &CSSParserQualifiedRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSParserQualifiedRule);

impl CSSParserQualifiedRule {
    /// Getter of the `prelude` attribute.
    /// [`CSSParserQualifiedRule.prelude`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserQualifiedRule/prelude)
    pub fn prelude(&self) -> TypedArray<CSSParserValue> {
        self.inner
            .get("prelude")
            .as_::<TypedArray<CSSParserValue>>()
    }
}
impl CSSParserQualifiedRule {
    /// Getter of the `body` attribute.
    /// [`CSSParserQualifiedRule.body`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserQualifiedRule/body)
    pub fn body(&self) -> TypedArray<CSSParserRule> {
        self.inner.get("body").as_::<TypedArray<CSSParserRule>>()
    }
}

impl CSSParserQualifiedRule {
    /// The `new CSSParserQualifiedRule(..)` constructor, creating a new CSSParserQualifiedRule instance
    pub fn new(prelude: &TypedArray<Any>) -> CSSParserQualifiedRule {
        Self {
            inner: Any::global("CSSParserQualifiedRule")
                .new(&[prelude.into()])
                .as_::<CSSParserRule>(),
        }
    }
}

impl CSSParserQualifiedRule {
    /// The `new CSSParserQualifiedRule(..)` constructor, creating a new CSSParserQualifiedRule instance
    pub fn new_with_body(
        prelude: &TypedArray<Any>,
        body: &TypedArray<CSSParserRule>,
    ) -> CSSParserQualifiedRule {
        Self {
            inner: Any::global("CSSParserQualifiedRule")
                .new(&[prelude.into(), body.into()])
                .as_::<CSSParserRule>(),
        }
    }
}
