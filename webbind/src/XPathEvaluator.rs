use super::*;




/// The XPathEvaluator class.
/// [`XPathEvaluator`](https://developer.mozilla.org/en-US/docs/Web/API/XPathEvaluator)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XPathEvaluator {
    inner: Any,
}

impl FromVal for XPathEvaluator {
    fn from_val(v: &Any) -> Self {
        XPathEvaluator { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XPathEvaluator {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XPathEvaluator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XPathEvaluator {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XPathEvaluator {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XPathEvaluator> for Any {
    fn from(s: XPathEvaluator) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XPathEvaluator> for Any {
    fn from(s: &XPathEvaluator) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XPathEvaluator);



impl XPathEvaluator {
    /// The `new XPathEvaluator(..)` constructor, creating a new XPathEvaluator instance
    pub fn new() -> XPathEvaluator {
        Self {
            inner: Any::global("XPathEvaluator").new(&[]).as_::<Any>(),
        }
    }

}
impl XPathEvaluator {
    /// The createExpression method.
    /// [`XPathEvaluator.createExpression`](https://developer.mozilla.org/en-US/docs/Web/API/XPathEvaluator/createExpression)
    pub fn create_expression0(&self, expression: &JsString) -> XPathExpression {
        self.inner.call("createExpression", &[expression.into(), ]).as_::<XPathExpression>()
    }
    /// The createExpression method.
    /// [`XPathEvaluator.createExpression`](https://developer.mozilla.org/en-US/docs/Web/API/XPathEvaluator/createExpression)
    pub fn create_expression1(&self, expression: &JsString, resolver: &Function) -> XPathExpression {
        self.inner.call("createExpression", &[expression.into(), resolver.into(), ]).as_::<XPathExpression>()
    }
}
impl XPathEvaluator {
    /// The createNSResolver method.
    /// [`XPathEvaluator.createNSResolver`](https://developer.mozilla.org/en-US/docs/Web/API/XPathEvaluator/createNSResolver)
    pub fn create_ns_resolver(&self, node_resolver: &Node) -> Node {
        self.inner.call("createNSResolver", &[node_resolver.into(), ]).as_::<Node>()
    }
}
impl XPathEvaluator {
    /// The evaluate method.
    /// [`XPathEvaluator.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/XPathEvaluator/evaluate)
    pub fn evaluate0(&self, expression: &JsString, context_node: &Node) -> XPathResult {
        self.inner.call("evaluate", &[expression.into(), context_node.into(), ]).as_::<XPathResult>()
    }
    /// The evaluate method.
    /// [`XPathEvaluator.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/XPathEvaluator/evaluate)
    pub fn evaluate1(&self, expression: &JsString, context_node: &Node, resolver: &Function) -> XPathResult {
        self.inner.call("evaluate", &[expression.into(), context_node.into(), resolver.into(), ]).as_::<XPathResult>()
    }
    /// The evaluate method.
    /// [`XPathEvaluator.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/XPathEvaluator/evaluate)
    pub fn evaluate2(&self, expression: &JsString, context_node: &Node, resolver: &Function, type_: u16) -> XPathResult {
        self.inner.call("evaluate", &[expression.into(), context_node.into(), resolver.into(), type_.into(), ]).as_::<XPathResult>()
    }
    /// The evaluate method.
    /// [`XPathEvaluator.evaluate`](https://developer.mozilla.org/en-US/docs/Web/API/XPathEvaluator/evaluate)
    pub fn evaluate3(&self, expression: &JsString, context_node: &Node, resolver: &Function, type_: u16, result: &XPathResult) -> XPathResult {
        self.inner.call("evaluate", &[expression.into(), context_node.into(), resolver.into(), type_.into(), result.into(), ]).as_::<XPathResult>()
    }
}
