use super::*;

/// The CSSMediaRule class.
/// [`CSSMediaRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMediaRule {
    inner: CSSConditionRule,
}

impl FromVal for CSSMediaRule {
    fn from_val(v: &Any) -> Self {
        CSSMediaRule {
            inner: CSSConditionRule::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSMediaRule {
    type Target = CSSConditionRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSMediaRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSMediaRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSMediaRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSMediaRule> for Any {
    fn from(s: CSSMediaRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSMediaRule> for Any {
    fn from(s: &CSSMediaRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSMediaRule);

impl CSSMediaRule {
    /// Getter of the `media` attribute.
    /// [`CSSMediaRule.media`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule/media)
    pub fn media(&self) -> MediaList {
        self.inner.get("media").as_::<MediaList>()
    }
}
impl CSSMediaRule {
    /// Getter of the `matches` attribute.
    /// [`CSSMediaRule.matches`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule/matches)
    pub fn matches(&self) -> bool {
        self.inner.get("matches").as_::<bool>()
    }
}
