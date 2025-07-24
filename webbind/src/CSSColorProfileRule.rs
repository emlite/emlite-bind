use super::*;

/// The CSSColorProfileRule class.
/// [`CSSColorProfileRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColorProfileRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSColorProfileRule {
    inner: CSSRule,
}
impl FromVal for CSSColorProfileRule {
    fn from_val(v: &Any) -> Self {
        CSSColorProfileRule {
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
impl core::ops::Deref for CSSColorProfileRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSColorProfileRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSColorProfileRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSColorProfileRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSColorProfileRule> for Any {
    fn from(s: CSSColorProfileRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSColorProfileRule> for Any {
    fn from(s: &CSSColorProfileRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSColorProfileRule);

impl CSSColorProfileRule {
    /// Getter of the `name` attribute.
    /// [`CSSColorProfileRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColorProfileRule/name)
    pub fn name(&self) -> CSSOMString {
        self.inner.get("name").as_::<CSSOMString>()
    }
}
impl CSSColorProfileRule {
    /// Getter of the `src` attribute.
    /// [`CSSColorProfileRule.src`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColorProfileRule/src)
    pub fn src(&self) -> CSSOMString {
        self.inner.get("src").as_::<CSSOMString>()
    }
}
impl CSSColorProfileRule {
    /// Getter of the `renderingIntent` attribute.
    /// [`CSSColorProfileRule.renderingIntent`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColorProfileRule/renderingIntent)
    pub fn rendering_intent(&self) -> CSSOMString {
        self.inner.get("renderingIntent").as_::<CSSOMString>()
    }
}
impl CSSColorProfileRule {
    /// Getter of the `components` attribute.
    /// [`CSSColorProfileRule.components`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColorProfileRule/components)
    pub fn components(&self) -> CSSOMString {
        self.inner.get("components").as_::<CSSOMString>()
    }
}
