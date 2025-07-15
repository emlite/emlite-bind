use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XPathExpression {
    inner: emlite::Val,
}
impl FromVal for XPathExpression {
    fn from_val(v: &emlite::Val) -> Self {
        XPathExpression {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XPathExpression {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XPathExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XPathExpression {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XPathExpression {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XPathExpression> for emlite::Val {
    fn from(s: XPathExpression) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XPathExpression> for emlite::Val {
    fn from(s: &XPathExpression) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XPathExpression);

impl XPathExpression {
    pub fn evaluate0(&self, context_node: &Node) -> XPathResult {
        self.inner
            .call("evaluate", &[context_node.into()])
            .as_::<XPathResult>()
    }

    pub fn evaluate1(&self, context_node: &Node, type_: u16) -> XPathResult {
        self.inner
            .call("evaluate", &[context_node.into(), type_.into()])
            .as_::<XPathResult>()
    }

    pub fn evaluate2(&self, context_node: &Node, type_: u16, result: &XPathResult) -> XPathResult {
        self.inner
            .call(
                "evaluate",
                &[context_node.into(), type_.into(), result.into()],
            )
            .as_::<XPathResult>()
    }
}
