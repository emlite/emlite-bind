use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FunctionParameter {
    inner: Any,
}
impl FromVal for FunctionParameter {
    fn from_val(v: &Any) -> Self {
        FunctionParameter { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FunctionParameter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FunctionParameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FunctionParameter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FunctionParameter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FunctionParameter> for Any {
    fn from(s: FunctionParameter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FunctionParameter> for Any {
    fn from(s: &FunctionParameter) -> Any {
        s.inner.clone()
    }
}

impl FunctionParameter {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl FunctionParameter {
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }

    pub fn set_type_(&mut self, value: &str) {
        self.inner.set("type", value);
    }
}
impl FunctionParameter {
    pub fn default_value(&self) -> String {
        self.inner.get("defaultValue").as_::<String>()
    }

    pub fn set_default_value(&mut self, value: &str) {
        self.inner.set("defaultValue", value);
    }
}
/// The CSSFunctionRule class.
/// [`CSSFunctionRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFunctionRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSFunctionRule {
    fn from_val(v: &Any) -> Self {
        CSSFunctionRule {
            inner: CSSGroupingRule::from_val(v),
        }
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
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl CSSFunctionRule {
    /// The getParameters method.
    /// [`CSSFunctionRule.getParameters`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionRule/getParameters)
    pub fn get_parameters(&self) -> Sequence<FunctionParameter> {
        self.inner
            .call("getParameters", &[])
            .as_::<Sequence<FunctionParameter>>()
    }
}
impl CSSFunctionRule {
    /// Getter of the `returnType` attribute.
    /// [`CSSFunctionRule.returnType`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFunctionRule/returnType)
    pub fn return_type(&self) -> String {
        self.inner.get("returnType").as_::<String>()
    }
}
