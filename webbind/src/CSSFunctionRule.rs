use super::*;




/// The CSSFunctionRule class.
/// [`CSSFunctionRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFunctionRule {
    inner: CSSGroupingRule,
}

impl FromVal for CSSFunctionRule {
    fn from_val(v: &Any) -> Self {
        CSSFunctionRule { inner: CSSGroupingRule::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSFunctionRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSFunctionRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSFunctionRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSFunctionRule {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSFunctionRule> for Any {
    fn from(s: CSSFunctionRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSFunctionRule> for Any {
    fn from(s: &CSSFunctionRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSFunctionRule);


impl CSSFunctionRule {
    /// Getter of the `name` attribute.
    /// [`CSSFunctionRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionRule/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

}
impl CSSFunctionRule {
    /// The getParameters method.
    /// [`CSSFunctionRule.getParameters`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionRule/getParameters)
    pub fn get_parameters(&self, ) -> TypedArray<FunctionParameter> {
        self.inner.call("getParameters", &[]).as_::<TypedArray<FunctionParameter>>()
    }
}
impl CSSFunctionRule {
    /// Getter of the `returnType` attribute.
    /// [`CSSFunctionRule.returnType`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionRule/returnType)
    pub fn return_type(&self) -> JsString {
        self.inner.get("returnType").as_::<JsString>()
    }

}
