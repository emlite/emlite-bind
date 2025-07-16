use super::*;

/// The CSSParserAtRule class.
/// [`CSSParserAtRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserAtRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSParserAtRule {
    inner: CSSParserRule,
}
impl FromVal for CSSParserAtRule {
    fn from_val(v: &Any) -> Self {
        CSSParserAtRule {
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
impl core::ops::Deref for CSSParserAtRule {
    type Target = CSSParserRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSParserAtRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSParserAtRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSParserAtRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSParserAtRule> for Any {
    fn from(s: CSSParserAtRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSParserAtRule> for Any {
    fn from(s: &CSSParserAtRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSParserAtRule);

impl CSSParserAtRule {
    /// The `new CSSParserAtRule(..)` constructor, creating a new CSSParserAtRule instance
    pub fn new0(name: &str, prelude: &Sequence<Any>) -> CSSParserAtRule {
        Self {
            inner: Any::global("CSSParserAtRule")
                .new(&[name.into(), prelude.into()])
                .as_::<CSSParserRule>(),
        }
    }

    /// The `new CSSParserAtRule(..)` constructor, creating a new CSSParserAtRule instance
    pub fn new1(
        name: &str,
        prelude: &Sequence<Any>,
        body: &Sequence<CSSParserRule>,
    ) -> CSSParserAtRule {
        Self {
            inner: Any::global("CSSParserAtRule")
                .new(&[name.into(), prelude.into(), body.into()])
                .as_::<CSSParserRule>(),
        }
    }
}
impl CSSParserAtRule {
    /// Getter of the `name` attribute.
    /// [`CSSParserAtRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserAtRule/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl CSSParserAtRule {
    /// Getter of the `prelude` attribute.
    /// [`CSSParserAtRule.prelude`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserAtRule/prelude)
    pub fn prelude(&self) -> FrozenArray<CSSParserValue> {
        self.inner
            .get("prelude")
            .as_::<FrozenArray<CSSParserValue>>()
    }
}
impl CSSParserAtRule {
    /// Getter of the `body` attribute.
    /// [`CSSParserAtRule.body`](https://developer.mozilla.org/en-US/docs/Web/API/CSSParserAtRule/body)
    pub fn body(&self) -> FrozenArray<CSSParserRule> {
        self.inner.get("body").as_::<FrozenArray<CSSParserRule>>()
    }
}
