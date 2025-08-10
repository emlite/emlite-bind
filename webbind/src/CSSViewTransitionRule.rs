use super::*;

/// The CSSViewTransitionRule class.
/// [`CSSViewTransitionRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSViewTransitionRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSViewTransitionRule {
    inner: CSSRule,
}

impl FromVal for CSSViewTransitionRule {
    fn from_val(v: &Any) -> Self {
        CSSViewTransitionRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSViewTransitionRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSViewTransitionRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSViewTransitionRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSViewTransitionRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSViewTransitionRule> for Any {
    fn from(s: CSSViewTransitionRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSViewTransitionRule> for Any {
    fn from(s: &CSSViewTransitionRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSViewTransitionRule);

impl CSSViewTransitionRule {
    /// Getter of the `navigation` attribute.
    /// [`CSSViewTransitionRule.navigation`](https://developer.mozilla.org/en-US/docs/Web/API/CSSViewTransitionRule/navigation)
    pub fn navigation(&self) -> JsString {
        self.inner.get("navigation").as_::<JsString>()
    }
}
impl CSSViewTransitionRule {
    /// Getter of the `types` attribute.
    /// [`CSSViewTransitionRule.types`](https://developer.mozilla.org/en-US/docs/Web/API/CSSViewTransitionRule/types)
    pub fn types(&self) -> TypedArray<JsString> {
        self.inner.get("types").as_::<TypedArray<JsString>>()
    }
}
