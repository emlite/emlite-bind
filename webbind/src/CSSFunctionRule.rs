use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FunctionParameter {
    inner: emlite::Val,
}
impl FromVal for FunctionParameter {
    fn from_val(v: &emlite::Val) -> Self {
        FunctionParameter { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FunctionParameter {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FunctionParameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FunctionParameter> for emlite::Val {
    fn from(s: FunctionParameter) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FunctionParameter {
    pub fn name(&self) -> jsbind::CSSOMString {
        self.inner.get("name").as_::<jsbind::CSSOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("name", value);
    }
}
impl FunctionParameter {
    pub fn type_(&self) -> jsbind::CSSOMString {
        self.inner.get("type").as_::<jsbind::CSSOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("type", value);
    }
}
impl FunctionParameter {
    pub fn default_value(&self) -> jsbind::CSSOMString {
        self.inner.get("defaultValue").as_::<jsbind::CSSOMString>()
    }

    pub fn set_default_value(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("defaultValue", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSFunctionRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSFunctionRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSFunctionRule {
            inner: CSSGroupingRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<CSSFunctionRule> for emlite::Val {
    fn from(s: CSSFunctionRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSFunctionRule {
    pub fn name(&self) -> jsbind::CSSOMString {
        self.inner.get("name").as_::<jsbind::CSSOMString>()
    }
}
impl CSSFunctionRule {
    pub fn get_parameters(&self) -> jsbind::Sequence<FunctionParameter> {
        self.inner
            .call("getParameters", &[])
            .as_::<jsbind::Sequence<FunctionParameter>>()
    }
}
impl CSSFunctionRule {
    pub fn return_type(&self) -> jsbind::CSSOMString {
        self.inner.get("returnType").as_::<jsbind::CSSOMString>()
    }
}
