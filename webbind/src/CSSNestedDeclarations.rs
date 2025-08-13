use super::*;




/// The CSSNestedDeclarations class.
/// [`CSSNestedDeclarations`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNestedDeclarations)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSNestedDeclarations {
    inner: CSSRule,
}

impl FromVal for CSSNestedDeclarations {
    fn from_val(v: &Any) -> Self {
        CSSNestedDeclarations { inner: CSSRule::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSNestedDeclarations {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSNestedDeclarations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSNestedDeclarations {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSNestedDeclarations {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSNestedDeclarations> for Any {
    fn from(s: CSSNestedDeclarations) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSNestedDeclarations> for Any {
    fn from(s: &CSSNestedDeclarations) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSNestedDeclarations);


impl CSSNestedDeclarations {
    /// Getter of the `style` attribute.
    /// [`CSSNestedDeclarations.style`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNestedDeclarations/style)
    pub fn style(&self) -> CSSStyleProperties {
        self.inner.get("style").as_::<CSSStyleProperties>()
    }

}
