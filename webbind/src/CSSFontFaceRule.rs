use super::*;

/// The CSSFontFaceRule class.
/// [`CSSFontFaceRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFontFaceRule {
    inner: CSSRule,
}

impl FromVal for CSSFontFaceRule {
    fn from_val(v: &Any) -> Self {
        CSSFontFaceRule {
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

impl core::ops::Deref for CSSFontFaceRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSFontFaceRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSFontFaceRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSFontFaceRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSFontFaceRule> for Any {
    fn from(s: CSSFontFaceRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSFontFaceRule> for Any {
    fn from(s: &CSSFontFaceRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSFontFaceRule);

impl CSSFontFaceRule {
    /// Getter of the `style` attribute.
    /// [`CSSFontFaceRule.style`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule/style)
    pub fn style(&self) -> CSSFontFaceDescriptors {
        self.inner.get("style").as_::<CSSFontFaceDescriptors>()
    }
}
