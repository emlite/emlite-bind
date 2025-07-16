use super::*;

/// The CSSStyleProperties class.
/// [`CSSStyleProperties`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleProperties)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleProperties {
    inner: CSSStyleDeclaration,
}
impl FromVal for CSSStyleProperties {
    fn from_val(v: &Any) -> Self {
        CSSStyleProperties {
            inner: CSSStyleDeclaration::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSStyleProperties {
    type Target = CSSStyleDeclaration;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSStyleProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSStyleProperties {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSStyleProperties {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSStyleProperties> for Any {
    fn from(s: CSSStyleProperties) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSStyleProperties> for Any {
    fn from(s: &CSSStyleProperties) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSStyleProperties);

impl CSSStyleProperties {
    /// Getter of the `cssFloat` attribute.
    /// [`CSSStyleProperties.cssFloat`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleProperties/cssFloat)
    pub fn css_float(&self) -> String {
        self.inner.get("cssFloat").as_::<String>()
    }

    /// Setter of the `cssFloat` attribute.
    /// [`CSSStyleProperties.cssFloat`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleProperties/cssFloat)
    pub fn set_css_float(&mut self, value: &str) {
        self.inner.set("cssFloat", value);
    }
}
