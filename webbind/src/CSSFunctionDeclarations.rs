use super::*;

/// The CSSFunctionDeclarations class.
/// [`CSSFunctionDeclarations`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionDeclarations)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFunctionDeclarations {
    inner: CSSRule,
}

impl FromVal for CSSFunctionDeclarations {
    fn from_val(v: &Any) -> Self {
        CSSFunctionDeclarations {
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

impl core::ops::Deref for CSSFunctionDeclarations {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSFunctionDeclarations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSFunctionDeclarations {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSFunctionDeclarations {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSFunctionDeclarations> for Any {
    fn from(s: CSSFunctionDeclarations) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSFunctionDeclarations> for Any {
    fn from(s: &CSSFunctionDeclarations) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSFunctionDeclarations);

impl CSSFunctionDeclarations {
    /// Getter of the `style` attribute.
    /// [`CSSFunctionDeclarations.style`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionDeclarations/style)
    pub fn style(&self) -> CSSFunctionDescriptors {
        self.inner.get("style").as_::<CSSFunctionDescriptors>()
    }
}
