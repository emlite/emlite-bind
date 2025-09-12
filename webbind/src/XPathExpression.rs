use super::*;

/// The XPathExpression class.
/// [`XPathExpression`](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XPathExpression {
    inner: Any,
}

impl FromVal for XPathExpression {
    fn from_val(v: &Any) -> Self {
        XPathExpression {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XPathExpression {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XPathExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XPathExpression {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XPathExpression {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XPathExpression> for Any {
    fn from(s: XPathExpression) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XPathExpression> for Any {
    fn from(s: &XPathExpression) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XPathExpression);

impl XPathExpression {
    /// The evaluate method.
    /// [`XPathExpression.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)
    pub fn evaluate(&self, context_node: &Node) -> XPathResult {
        self.inner
            .call("evaluate", &[context_node.into()])
            .as_::<XPathResult>()
    }
}
impl XPathExpression {
    /// The evaluate method.
    /// [`XPathExpression.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)
    pub fn evaluate_with_type(&self, context_node: &Node, type_: u16) -> XPathResult {
        self.inner
            .call("evaluate", &[context_node.into(), type_.into()])
            .as_::<XPathResult>()
    }
}
impl XPathExpression {
    /// The evaluate method.
    /// [`XPathExpression.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/XPathExpression/evaluate)
    pub fn evaluate_with_type_and_result(
        &self,
        context_node: &Node,
        type_: u16,
        result: &XPathResult,
    ) -> XPathResult {
        self.inner
            .call(
                "evaluate",
                &[context_node.into(), type_.into(), result.into()],
            )
            .as_::<XPathResult>()
    }
}
