use super::*;




/// The CSSLayerStatementRule class.
/// [`CSSLayerStatementRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLayerStatementRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSLayerStatementRule {
    inner: CSSRule,
}

impl FromVal for CSSLayerStatementRule {
    fn from_val(v: &Any) -> Self {
        CSSLayerStatementRule { inner: CSSRule::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSLayerStatementRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSLayerStatementRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSLayerStatementRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSLayerStatementRule {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSLayerStatementRule> for Any {
    fn from(s: CSSLayerStatementRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSLayerStatementRule> for Any {
    fn from(s: &CSSLayerStatementRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSLayerStatementRule);


impl CSSLayerStatementRule {
    /// Getter of the `nameList` attribute.
    /// [`CSSLayerStatementRule.nameList`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLayerStatementRule/nameList)
    pub fn name_list(&self) -> TypedArray<JsString> {
        self.inner.get("nameList").as_::<TypedArray<JsString>>()
    }

}
